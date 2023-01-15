use assert_cmd::Command;
use e2e_test_context::E2ETestContext;
use predicates::prelude::*;
use test_context::test_context;

mod e2e_test_context;

#[test_context(E2ETestContext)]
#[test]
fn test_list_files_success(ctx: &mut E2ETestContext) {
    ctx.start_server();
    ctx.create_test_files("abc", "hello");
    ctx.create_test_files("xyz", "grpc");

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
