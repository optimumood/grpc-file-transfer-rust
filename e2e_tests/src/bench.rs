use crate::{
    e2e_test_context::{AppType, E2ETestContext},
    utils::get_base_client_cmd,
};
use assert_cmd::Command;
use bytesize::ByteSize;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use predicates::prelude::predicate;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::path::{Path, PathBuf};

mod e2e_test_context;
mod utils;

fn list_files(cmd: &mut Command) {
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File name  Size"))
        .stdout(predicate::str::contains("abc        5B"))
        .stdout(predicate::str::contains("xyz        4B"));
}

fn download_file(cmd: &mut Command, files: (&PathBuf, &PathBuf)) {
    cmd.assert().success();

    compare_files_on_disk(files.0, files.1);
}

fn upload_file(cmd: &mut Command, files: (&PathBuf, &PathBuf)) {
    cmd.assert().success();

    compare_files_on_disk(files.0, files.1);
}

fn compare_files_on_disk(left: &Path, right: &Path) {
    let status = std::process::Command::new("diff")
        .arg(left.as_os_str())
        .arg(right.as_os_str())
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .expect("failed to execute diff process");

    if !status.success() {
        panic!("diff for file failed");
    }
}

fn criterion_benchmark_list_files(c: &mut Criterion) {
    let mut ctx = E2ETestContext::setup();
    ctx.start_server("::1".parse().unwrap(), false);
    ctx.create_test_file(AppType::Server, "abc", "hello");
    ctx.create_test_file(AppType::Server, "xyz", "grpc");

    let mut cmd = get_base_client_cmd(&ctx, &"::1".parse().unwrap(), false);
    cmd.arg("list");

    let mut group = c.benchmark_group("throughput-list_files");
    group.throughput(Throughput::Elements(1));
    group.bench_function("list_files", |b| b.iter(|| list_files(black_box(&mut cmd))));
    group.finish();
}

fn criterion_benchmark_list_files_tls(c: &mut Criterion) {
    let mut ctx = E2ETestContext::setup();
    ctx.gen_all_creds();
    ctx.start_server("::1".parse().unwrap(), true);
    ctx.create_test_file(AppType::Server, "abc", "hello");
    ctx.create_test_file(AppType::Server, "xyz", "grpc");

    let mut cmd = get_base_client_cmd(&ctx, &"::1".parse().unwrap(), true);
    cmd.arg("list");

    let mut group = c.benchmark_group("throughput-list_files_tls");
    group.throughput(Throughput::Elements(1));
    group.bench_function("list_files", |b| b.iter(|| list_files(black_box(&mut cmd))));
    group.finish();
}

fn criterion_benchmark_download_file(c: &mut Criterion) {
    let mut ctx = E2ETestContext::setup();
    ctx.start_server("::1".parse().unwrap(), false);

    let mut group = c.benchmark_group("throughput-download_file");

    for size in [ByteSize::mib(1), ByteSize::gib(1)] {
        let rand_content: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(size.as_u64() as usize)
            .map(char::from)
            .collect();

        let file_name = format!("file_{size}");
        ctx.create_test_file(AppType::Server, &file_name, &rand_content);

        let file_server_path = ctx.server.files.last().unwrap().abs_path.clone();

        let mut file_client_path = PathBuf::new();
        file_client_path.push(ctx.client.dir.path());
        file_client_path.push(&file_name);

        let mut download_cmd = get_base_client_cmd(&ctx, &"::1".parse().unwrap(), false);
        download_cmd
            .arg("download")
            .args(["--file", &file_name])
            .args(["--directory", ctx.client.dir.path().to_str().unwrap()]);

        group.throughput(Throughput::Bytes(size.as_u64()));
        group.sample_size(10);

        group.bench_function(format!("download_file_{}", size.to_string_as(true)), |b| {
            b.iter(|| {
                download_file(
                    black_box(&mut download_cmd),
                    (&file_server_path, &file_client_path),
                )
            })
        });
    }

    group.finish();
}

fn criterion_benchmark_download_file_tls(c: &mut Criterion) {
    let mut ctx = E2ETestContext::setup();
    ctx.gen_all_creds();
    ctx.start_server("::1".parse().unwrap(), true);

    let mut group = c.benchmark_group("throughput-download_file_tls");

    for size in [ByteSize::mib(1), ByteSize::gib(1)] {
        let rand_content: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(size.as_u64() as usize)
            .map(char::from)
            .collect();

        let file_name = format!("file_{size}");
        ctx.create_test_file(AppType::Server, &file_name, &rand_content);

        let file_server_path = ctx.server.files.last().unwrap().abs_path.clone();

        let mut file_client_path = PathBuf::new();
        file_client_path.push(ctx.client.dir.path());
        file_client_path.push(&file_name);

        let mut download_cmd = get_base_client_cmd(&ctx, &"::1".parse().unwrap(), true);
        download_cmd
            .arg("download")
            .args(["--file", &file_name])
            .args(["--directory", ctx.client.dir.path().to_str().unwrap()]);

        group.throughput(Throughput::Bytes(size.as_u64()));
        group.sample_size(10);

        group.bench_function(format!("download_file_{}", size.to_string_as(true)), |b| {
            b.iter(|| {
                download_file(
                    black_box(&mut download_cmd),
                    (&file_server_path, &file_client_path),
                )
            })
        });
    }

    group.finish();
}

fn criterion_benchmark_upload_file(c: &mut Criterion) {
    let mut ctx = E2ETestContext::setup();
    ctx.start_server("::1".parse().unwrap(), false);

    let mut group = c.benchmark_group("throughput-upload_file");

    for size in [ByteSize::mib(1), ByteSize::gib(1)] {
        let rand_content: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(size.as_u64() as usize)
            .map(char::from)
            .collect();

        let file_name = format!("file_{size}");
        ctx.create_test_file(AppType::Client, &file_name, &rand_content);

        let mut file_server_path = PathBuf::new();
        file_server_path.push(ctx.server.dir.path());
        file_server_path.push(&file_name);

        let file_client_path = ctx.client.files.last().unwrap().abs_path.clone();

        let mut upload_cmd = get_base_client_cmd(&ctx, &"::1".parse().unwrap(), false);
        upload_cmd
            .arg("upload")
            .args(["--file", &file_name])
            .args(["--directory", ctx.client.dir.path().to_str().unwrap()]);

        group.throughput(Throughput::Bytes(size.as_u64()));
        group.sample_size(10);

        group.bench_function(format!("upload_file_{}", size.to_string_as(true)), |b| {
            b.iter(|| {
                upload_file(
                    black_box(&mut upload_cmd),
                    (&file_server_path, &file_client_path),
                )
            })
        });
    }

    group.finish();
}

fn criterion_benchmark_upload_file_tls(c: &mut Criterion) {
    let mut ctx = E2ETestContext::setup();
    ctx.gen_all_creds();
    ctx.start_server("::1".parse().unwrap(), true);

    let mut group = c.benchmark_group("throughput-upload_file_tls");

    for size in [ByteSize::mib(1), ByteSize::gib(1)] {
        let rand_content: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(size.as_u64() as usize)
            .map(char::from)
            .collect();

        let file_name = format!("file_{size}");
        ctx.create_test_file(AppType::Client, &file_name, &rand_content);

        let mut file_server_path = PathBuf::new();
        file_server_path.push(ctx.server.dir.path());
        file_server_path.push(&file_name);

        let file_client_path = ctx.client.files.last().unwrap().abs_path.clone();

        let mut upload_cmd = get_base_client_cmd(&ctx, &"::1".parse().unwrap(), true);
        upload_cmd
            .arg("upload")
            .args(["--file", &file_name])
            .args(["--directory", ctx.client.dir.path().to_str().unwrap()]);

        group.throughput(Throughput::Bytes(size.as_u64()));
        group.sample_size(10);

        group.bench_function(format!("upload_file_{}", size.to_string_as(true)), |b| {
            b.iter(|| {
                upload_file(
                    black_box(&mut upload_cmd),
                    (&file_server_path, &file_client_path),
                )
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    criterion_benchmark_list_files,
    criterion_benchmark_download_file,
    criterion_benchmark_upload_file,
);
criterion_group!(
    benches_tls,
    criterion_benchmark_list_files_tls,
    criterion_benchmark_download_file_tls,
    criterion_benchmark_upload_file_tls
);
criterion_main!(benches, benches_tls);
