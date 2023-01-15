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
    pub server_dir: TempDir,
    pub client_dir: TempDir,
    pub port: u16,
    pub server_child: Option<Child>,
    pub files: Vec<File>,
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

        E2ETestContext {
            server_dir,
            client_dir,
            port,
            server_child: None,
            files: vec![],
        }
    }

    fn teardown(self) {
        if let Some(mut server_child) = self.server_child {
            if let Err(err) = server_child.kill() {
                eprintln!("failed to kill server process: {err}");
            }
        }

        if let Err(err) = self.server_dir.close() {
            eprintln!("failed to delete temporary server directory: {err}");
        }

        if let Err(err) = self.client_dir.close() {
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
            .args(["--directory", self.server_dir.path().to_str().unwrap()])
            .spawn()
            .expect("server failed to start");

        self.server_child = Some(server_child);
    }

    pub fn create_test_files(&mut self, name: &str, content: &str) {
        let mut file_path = PathBuf::new();
        file_path.push(self.server_dir.path());
        file_path.push(name);

        let mut test_file = File::create(file_path).expect("failed to create test file");

        test_file
            .write_all(content.as_bytes())
            .expect("write_all failed");
        test_file.sync_all().expect("sync_all failed");

        self.files.push(test_file);
    }
}
