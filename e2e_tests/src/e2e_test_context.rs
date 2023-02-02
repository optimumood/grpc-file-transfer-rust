use assert_cmd::cargo::cargo_bin;
use rcgen::generate_simple_self_signed;
use rstest::*;
use std::fs::{self, File};
use std::io::Write;
use std::mem::ManuallyDrop;
use std::net::IpAddr;
use std::path::PathBuf;
use std::process::{Child, Command};
use tempdir::TempDir;
use tokio::runtime::Runtime;
use tonic::transport::Channel;

pub struct E2ETestContext {
    pub client: Client,
    pub server: Server,
    pub port: u16,
}

pub struct Client {
    pub dir: ManuallyDrop<TempDir>,
    pub files: Vec<TestFile>,
    pub ca_cert: Option<PathBuf>,
}

pub struct Server {
    pub dir: ManuallyDrop<TempDir>,
    pub process: Option<Child>,
    pub files: Vec<TestFile>,
    pub cert: Option<PathBuf>,
    pub key: Option<PathBuf>,
}

pub struct TestFile {
    pub handle: File,
    pub abs_path: PathBuf,
}

pub enum AppType {
    Client,
    Server,
}

#[fixture]
pub fn ctx() -> E2ETestContext {
    E2ETestContext::setup()
}

impl E2ETestContext {
    pub fn setup() -> E2ETestContext {
        let server_dir =
            TempDir::new("my_directory_prefix").expect("Failed to create temporary directory");
        let client_dir =
            TempDir::new("my_directory_prefix").expect("Failed to create temporary directory");

        let port = portpicker::pick_unused_port().expect("No ports free");

        let server = Server {
            dir: ManuallyDrop::new(server_dir),
            files: vec![],
            process: None,
            cert: None,
            key: None,
        };
        let client = Client {
            dir: ManuallyDrop::new(client_dir),
            files: vec![],
            ca_cert: None,
        };

        E2ETestContext {
            client,
            server,
            port,
        }
    }

    fn teardown(&mut self) {
        if let Some(ref mut process) = self.server.process {
            if let Err(err) = process.kill() {
                eprintln!("failed to kill server process: {err}");
            }
        }
        let server_dir;
        unsafe {
            server_dir = ManuallyDrop::take(&mut self.server.dir);
        }
        if let Err(err) = server_dir.close() {
            eprintln!("failed to delete temporary server directory: {err}");
        }

        let client_dir;
        unsafe {
            client_dir = ManuallyDrop::take(&mut self.client.dir);
        }
        if let Err(err) = client_dir.close() {
            eprintln!("failed to delete temporary client directory: {err}");
        }
    }
}

impl Drop for E2ETestContext {
    fn drop(&mut self) {
        self.teardown();
    }
}

impl E2ETestContext {
    const SERVER_BIN_NAME: &str = "server";
    const CA_CERT_NAME: &str = "ca-cert.pem";
    const SERVER_CERT_NAME: &str = "server-cert.pem";
    const SERVER_KEY_NAME: &str = "server-key.pem";
    const CERTS_DIR: &str = "certs";

    pub fn start_server(&mut self, server_ip_address: IpAddr, tls: bool) {
        let server_bin_path = cargo_bin(Self::SERVER_BIN_NAME);

        let mut server_cmd = Command::new(server_bin_path);
        server_cmd
            .args(["--port", &self.port.to_string()])
            .args(["--address", &server_ip_address.to_string()])
            .args(["--directory", self.server.dir.path().to_str().unwrap()]);

        if tls {
            server_cmd.args(["--key", self.server.key.as_ref().unwrap().to_str().unwrap()]);
            server_cmd.args([
                "--cert",
                self.server.cert.as_ref().unwrap().to_str().unwrap(),
            ]);
        } else {
            server_cmd.arg("--insecure");
        }

        let server_child = server_cmd.spawn().expect("server failed to start");

        self.wait_for_server(&server_ip_address);

        self.server.process = Some(server_child);
    }

    fn wait_for_server(&self, server_ip_address: &IpAddr) {
        let rt = Runtime::new().unwrap();
        let server_address = match server_ip_address {
            IpAddr::V4(ipv4) => format!("http://{}:{}", ipv4, self.port),
            IpAddr::V6(ipv6) => format!("http://[{}]:{}", ipv6, self.port),
        };
        let mut retries: u32 = 0;
        const MAX_RETRIES: u32 = 10;

        loop {
            retries += 1;

            if let Err(err) = rt.block_on(async {
                Channel::builder(server_address.parse().unwrap())
                    .connect()
                    .await?;

                Ok::<(), tonic::transport::Error>(())
            }) {
                eprintln!("Couldn't connect to server: {err}");
                std::thread::sleep(std::time::Duration::from_secs(1));
            } else {
                break;
            }

            if retries >= MAX_RETRIES {
                panic!("Couldn't connect to server");
            }
        }
    }

    pub fn create_test_file(&mut self, app_type: AppType, file_name: &str, file_content: &str) {
        let mut file_path = PathBuf::new();
        match app_type {
            AppType::Client => file_path.push(self.client.dir.path()),
            AppType::Server => file_path.push(self.server.dir.path()),
        }
        file_path.push(file_name);

        let mut file_handle = File::create(&file_path).expect("failed to create test file");

        file_handle
            .write_all(file_content.as_bytes())
            .expect("write_all failed");
        file_handle.sync_all().expect("sync_all failed");

        let test_file = TestFile {
            handle: file_handle,
            abs_path: file_path,
        };
        match app_type {
            AppType::Client => self.client.files.push(test_file),
            AppType::Server => self.server.files.push(test_file),
        }
    }

    pub fn gen_certs(&mut self) {
        let subject_alt_names = vec!["localhost".to_string()];
        let cert = generate_simple_self_signed(subject_alt_names).unwrap();

        let mut client_certs_path = PathBuf::from(self.client.dir.path());
        client_certs_path.push(Self::CERTS_DIR);
        fs::create_dir(&client_certs_path).unwrap();

        let mut server_certs_path = PathBuf::from(self.server.dir.path());
        server_certs_path.push(Self::CERTS_DIR);
        fs::create_dir(&server_certs_path).unwrap();

        let mut client_ca_cert_path = client_certs_path.clone();
        client_ca_cert_path.push(Self::CA_CERT_NAME);
        fs::write(&client_ca_cert_path, cert.serialize_pem().unwrap()).unwrap();
        self.client.ca_cert = Some(client_ca_cert_path);

        let mut server_cert_path = server_certs_path.clone();
        server_cert_path.push(Self::SERVER_CERT_NAME);
        fs::write(&server_cert_path, cert.serialize_pem().unwrap()).unwrap();
        self.server.cert = Some(server_cert_path);

        let mut server_key_path = server_certs_path.clone();
        server_key_path.push(Self::SERVER_KEY_NAME);
        fs::write(&server_key_path, cert.serialize_private_key_pem()).unwrap();
        self.server.key = Some(server_key_path);
    }
}
