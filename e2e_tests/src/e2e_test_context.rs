use assert_cmd::cargo::cargo_bin;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Once;
use tempdir::TempDir;
use test_context::TestContext;

static BUILD: Once = Once::new();

pub struct E2ETestContext {
    pub client: Client,
    pub server: Server,
    pub port: u16,
}

pub struct Client {
    pub dir: TempDir,
    pub files: Vec<TestFile>,
}

pub struct Server {
    pub dir: TempDir,
    pub process: Option<Child>,
    pub files: Vec<TestFile>,
}

pub struct TestFile {
    pub handle: File,
    pub abs_path: PathBuf,
}

pub enum AppType {
    Client,
    Server,
}

impl TestContext for E2ETestContext {
    fn setup() -> E2ETestContext {
        BUILD.call_once(|| {
            Command::new("cargo")
                .arg("build")
                .arg("--workspace")
                .spawn()
                .expect("failed to build")
                .wait()
                .expect("failed waiting for build process");
        });

        let server_dir =
            TempDir::new("my_directory_prefix").expect("Failed to create temporary directory");
        let client_dir =
            TempDir::new("my_directory_prefix").expect("Failed to create temporary directory");

        let port = portpicker::pick_unused_port().expect("No ports free");

        let server = Server {
            dir: server_dir,
            files: vec![],
            process: None,
        };
        let client = Client {
            dir: client_dir,
            files: vec![],
        };

        E2ETestContext {
            client,
            server,
            port,
        }
    }

    fn teardown(self) {
        if let Some(mut process) = self.server.process {
            if let Err(err) = process.kill() {
                eprintln!("failed to kill server process: {err}");
            }
        }

        if let Err(err) = self.server.dir.close() {
            eprintln!("failed to delete temporary server directory: {err}");
        }

        if let Err(err) = self.client.dir.close() {
            eprintln!("failed to delete temporary client directory: {err}");
        }
    }
}

impl E2ETestContext {
    const SERVER_BIN_NAME: &str = "server";

    pub fn start_server(&mut self) {
        let server_bin_path = cargo_bin(Self::SERVER_BIN_NAME);
        let server_child = Command::new(server_bin_path)
            .args(["--port", &self.port.to_string()])
            .args(["--address", "::1"])
            .args(["--directory", self.server.dir.path().to_str().unwrap()])
            .spawn()
            .expect("server failed to start");

        self.server.process = Some(server_child);
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
}
