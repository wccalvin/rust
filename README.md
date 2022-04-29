# Rust

This project contains notes and exercises to get started on `rust`

## Install

Using [rustup](https://www.rust-lang.org/tools/install)

## Table of contents

| chapter | description | content |
| - | - | - |
| 00 | rust intro | |
| 01 | compile with rustc | [01-rust-compile](https://github.com/wccalvin/rust/tree/main/01-rust-compile) |

1. Use `cargo`

Initialize the directory using `cargo init`. It will generate a `Cargo.toml` file, `.gitignore` file and a `src` directory.

To execute: `cargo run` (the executable will be available at `./target/debug`)
To execute the previously compiled version: `./target/debug/rust`
To just build and not execute: `cargo build`
To build for production: `cargo build --release` (the release executable will be availabe at `./target/release`)
