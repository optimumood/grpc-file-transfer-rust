use assert_cmd::Command;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use e2e_test_context::{AppType, E2ETestContext};

mod e2e_test_context;

fn list_files(cmd: &mut Command) {
    cmd.assert().success();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut ctx = E2ETestContext::setup();
    ctx.start_server("::1".parse().unwrap());
    ctx.create_test_file(AppType::Server, "abc", "hello");
    ctx.create_test_file(AppType::Server, "xyz", "grpc");

    let mut cmd = Command::cargo_bin("client").unwrap();
    cmd.args(["--port", &ctx.port.to_string()])
        .args(["--address", "::1"])
        .arg("list");

    let mut group = c.benchmark_group("throughput-list_files");
    group.throughput(Throughput::Elements(1));
    group.bench_function("list_files", |b| b.iter(|| list_files(black_box(&mut cmd))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
