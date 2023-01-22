# grpc-file-transfer-rust

[![Status](https://img.shields.io/github/actions/workflow/status/optimumood/grpc-file-transfer-rust/rust.yml?branch=main)](https://github.com/optimumood/grpc-file-transfer-rust/actions/workflows/rust.yml)
[![License](https://img.shields.io/github/license/optimumood/grpc-file-transfer-rust)](/LICENSE)

---

> gRPC file server and client written in Rust language.

## :scroll: Table of Contents
- [:thinking: About](#about)
- [:rocket: Getting Started](#getting-started)
    - [:shopping_cart: Prerequisites](#prerequisites)
    - [:hammer: Building binaries](#building-binaries)
    - [:electric_plug: Installing binaries](#installing-binaries)
    - [:heavy_check_mark: Running end-to-end tests](#running-e2e-tests)
    - [:turtle: Running end-to-end benchmarks](#running-e2e-benchmarks)
    - [:triangular_flag_on_post: Automated workflow](#automated-workflow)
    - [:desktop_computer: Usage](#usage)
- [:building_construction: Technology stack](#technology-stack)

## :thinking: About <a name = "about"></a>
This project consists of gRPC server and client applications.

Available features:
- list available files on server
- upload files to server
- download files from server

## :rocket: Getting started <a name = "getting-started"></a>
Read:
- how to build and install binaries
- how to run end-to-end tests and benchmarks
- about project's automated workflow
- about usage

### :shopping_cart: Prerequisites <a name = "prerequisites"></a>
If you want to build and run applications or tests, you need to install:
- [Rust toolchain](https://www.rust-lang.org)
- [Cargo-make](https://sagiegurari.github.io/cargo-make/) (Rust task runner and build tool)

### :hammer: Building binaries <a name = "building-binaries"></a>
#### debug mode
```shell
cargo make build
```

#### release mode
```shell
cargo make build-release
```

### :electric_plug: Installing binaries <a name = "installing-binaries"></a>
#### server
```shell
cargo make install-server
```
#### client
```shell
cargo make install-client
```

### :heavy_check_mark: Running end-to-end tests <a name = "running-e2e-tests"></a>

```shell
cargo make e2e-tests
```

### :turtle: Running end-to-end benchmarks <a name = "running-e2e-benchmarks"></a>
```shell
cargo make e2e-bench
```

### :triangular_flag_on_post: Automated workflow <a name = "automated-workflow"></a>
This project has GitHub Actions workflow, which:
- checks formatting
- checks lints
- checks if client and server binaries are building
- runs end-to-end tests
- runs end-to-end benchmarks

### :desktop_computer: Usage <a name="usage"></a>
First, install server and client applications using [Installing binaries](#installing-binaries) instruction.\
Now, you can use applications.\
Below are presented example commands.

- Server help command
```shell
$ server --help
Usage: server [OPTIONS] --directory <DIRECTORY>

Options:
  -d, --directory <DIRECTORY>
  -H, --address <ADDRESS>      [default: 127.0.0.1]
  -p, --port <PORT>
  -v, --verbose <VERBOSE>      [default: info]
  -h, --help                   Print help
  -V, --version                Print version
```

- Client help command
```shell
$ client --help
Usage: client [OPTIONS] --port <PORT> <COMMAND>

Commands:
  download
  upload
  list
  help      Print this message or the help of the given subcommand(s)

Options:
  -H, --address <ADDRESS>  [default: 127.0.0.1]
  -p, --port <PORT>
  -v, --verbose <VERBOSE>  [default: info]
  -h, --help               Print help
  -V, --version            Print version
```

- Run server on IPv6 localhost address with 50051 port and /tmp/server path as server directory.
```shell
$ server --directory /tmp/server -p 50051 --address ::1
```

- List files command
```shell
$ client --port 50051 --address ::1 list
 File name  Size
 abc        12B
 abc2       0B
```

## :building_construction: Technology stack <a name = "technology-stack"></a>
- [Rust](https://www.rust-lang.org/) - Programming language
- [Tonic](https://github.com/hyperium/tonic) - Asynchronous Rust implementation of gRPC
- [PROST!](https://docs.rs/prost/latest/prost/) - Protocol Buffers implementation for the Rust Language
- [Tokio](https://tokio.rs/) - Asynchronous runtime for Rust
