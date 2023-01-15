use assert_cmd::Command;
use e2e_test_context::{AppType, E2ETestContext};
use predicates::prelude::*;
use std::path::PathBuf;
use test_context::test_context;
use utils::compare_files;

mod e2e_test_context;
mod utils;

#[test_context(E2ETestContext)]
#[test]
fn test_list_files_success(ctx: &mut E2ETestContext) {
    ctx.start_server();
    ctx.create_test_file(AppType::Server, "abc", "hello");
    ctx.create_test_file(AppType::Server, "xyz", "grpc");

    let mut cmd = Command::cargo_bin("client").unwrap();
    let assert = cmd
        .args(["--port", &ctx.port.to_string()])
        .args(["--address", "::1"])
        .arg("list")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("File name  Size"))
        .stdout(predicate::str::contains("abc        5B"))
        .stdout(predicate::str::contains("xyz        4B"));
}

#[test_context(E2ETestContext)]
#[test]
fn test_list_files_empty_success(ctx: &mut E2ETestContext) {
    ctx.start_server();

    let mut cmd = Command::cargo_bin("client").unwrap();
    let assert = cmd
        .args(["--port", &ctx.port.to_string()])
        .args(["--address", "::1"])
        .arg("list")
        .assert();

    assert
        .success()
        .stdout(predicate::str::ends_with("File name  Size").trim());
}

#[test_context(E2ETestContext)]
#[test]
fn test_download_file_success(ctx: &mut E2ETestContext) {
    ctx.start_server();

    let test_file_name = "abc";
    ctx.create_test_file(AppType::Server, test_file_name, "hello");

    let mut cmd = Command::cargo_bin("client").unwrap();
    let result = cmd
        .args(["--port", &ctx.port.to_string()])
        .args(["--address", "::1"])
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

#[test_context(E2ETestContext)]
#[test]
fn test_upload_file_success(ctx: &mut E2ETestContext) {
    ctx.start_server();

    let test_file_name = "abc";
    ctx.create_test_file(AppType::Client, test_file_name, "hello");

    let mut cmd = Command::cargo_bin("client").unwrap();
    let result = cmd
        .args(["--port", &ctx.port.to_string()])
        .args(["--address", "::1"])
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
