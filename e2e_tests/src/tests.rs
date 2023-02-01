use crate::e2e_test_context::ctx;
use assert_cmd::Command;
use e2e_test_context::{AppType, E2ETestContext};
use predicates::prelude::*;
use rstest::*;
use std::net::IpAddr;
use std::path::PathBuf;
use utils::compare_files;

mod e2e_test_context;
mod utils;

fn get_base_client_cmd(ctx: &E2ETestContext, ip_address: &IpAddr) -> Command {
    let mut cmd = Command::cargo_bin("client").unwrap();
    cmd.args(["--port", &ctx.port.to_string()])
        .args(["--address", &ip_address.to_string()])
        .arg("--insecure");
    cmd
}

#[rstest]
#[case::ipv4("0.0.0.0")]
#[case::ipv6("::1")]
fn test_list_files_success(mut ctx: E2ETestContext, #[case] ip_address: IpAddr) {
    ctx.start_server(ip_address);
    ctx.create_test_file(AppType::Server, "abc", "hello");
    ctx.create_test_file(AppType::Server, "xyz", "grpc");

    let mut cmd = get_base_client_cmd(&ctx, &ip_address);
    let assert = cmd.arg("list").assert();

    assert
        .success()
        .stdout(predicate::str::contains("File name  Size"))
        .stdout(predicate::str::contains("abc        5B"))
        .stdout(predicate::str::contains("xyz        4B"));
}

#[rstest]
#[case::ipv4("0.0.0.0")]
#[case::ipv6("::1")]
fn test_list_files_empty_success(mut ctx: E2ETestContext, #[case] ip_address: IpAddr) {
    ctx.start_server(ip_address);

    let mut cmd = get_base_client_cmd(&ctx, &ip_address);
    let assert = cmd.arg("list").assert();

    assert
        .success()
        .stdout(predicate::str::ends_with("File name  Size").trim());
}

#[rstest]
#[case::ipv4("0.0.0.0")]
#[case::ipv6("::1")]
fn test_download_file_success(mut ctx: E2ETestContext, #[case] ip_address: IpAddr) {
    ctx.start_server(ip_address);

    let test_file_name = "abc";
    ctx.create_test_file(AppType::Server, test_file_name, "hello");

    let mut cmd = get_base_client_cmd(&ctx, &ip_address);
    let result = cmd
        .arg("download")
        .args(["--file", test_file_name])
        .args(["--directory", ctx.client.dir.path().to_str().unwrap()])
        .ok();

    assert!(result.is_ok());

    let mut expected_client_file_path = PathBuf::new();
    expected_client_file_path.push(ctx.client.dir.path());
    expected_client_file_path.push(test_file_name);

    assert!(expected_client_file_path.exists());

    assert!(compare_files(
        &expected_client_file_path,
        &ctx.server.files[0].abs_path
    ));
}

#[rstest]
#[case::ipv4("0.0.0.0")]
#[case::ipv6("::1")]
fn test_upload_file_success(mut ctx: E2ETestContext, #[case] ip_address: IpAddr) {
    ctx.start_server(ip_address);

    let test_file_name = "abc";
    ctx.create_test_file(AppType::Client, test_file_name, "hello");

    let mut cmd = get_base_client_cmd(&ctx, &ip_address);
    let result = cmd
        .arg("upload")
        .args(["--file", test_file_name])
        .args(["--directory", ctx.client.dir.path().to_str().unwrap()])
        .ok();

    assert!(result.is_ok());

    let mut expected_server_file_path = PathBuf::new();
    expected_server_file_path.push(ctx.server.dir.path());
    expected_server_file_path.push(test_file_name);

    assert!(expected_server_file_path.exists());

    assert!(compare_files(
        &expected_server_file_path,
        &ctx.client.files[0].abs_path,
    ));
}
