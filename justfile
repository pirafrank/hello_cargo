#!/usr/bin/env -S just

# Task to uild the project
build:
    cargo build

# Task to build the project in release mode
release:
    cargo build --release

# Task to run the project
hi:
    cargo run -- hi --name "John Doe"

# Another task to run the project
hello:
    cargo run -- hello --name "Jane Park"

# Task to run tests
test:
    cargo test --verbose

help:
    cargo run -- --help
