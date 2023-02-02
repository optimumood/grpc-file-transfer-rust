use crate::e2e_test_context::E2ETestContext;
use assert_cmd::Command;
use std::{fs, net::IpAddr, path::PathBuf};

pub fn compare_files(left: &PathBuf, right: &PathBuf) -> bool {
    let left_data = fs::read(left).expect("Failed to read left file");
    let right_data = fs::read(right).expect("Failed to read right file");

    left_data == right_data
}

pub fn get_base_client_cmd(ctx: &E2ETestContext, ip_address: &IpAddr, tls: bool) -> Command {
    let mut cmd = Command::cargo_bin("client").unwrap();
    cmd.args(["--port", &ctx.port.to_string()]);

    if tls {
        cmd.args(["--address", "localhost"])
            .args([
                "--ca-cert",
                ctx.server.creds.as_ref().unwrap().ca_cert.to_str().unwrap(),
            ])
            .args([
                "--key",
                ctx.client
                    .creds
                    .as_ref()
                    .unwrap()
                    .identity
                    .key
                    .as_path()
                    .to_str()
                    .unwrap(),
            ])
            .args([
                "--cert",
                ctx.client
                    .creds
                    .as_ref()
                    .unwrap()
                    .identity
                    .cert
                    .as_path()
                    .to_str()
                    .unwrap(),
            ]);
    } else {
        cmd.args(["--address", &ip_address.to_string()])
            .arg("--insecure");
    }

    cmd
}
