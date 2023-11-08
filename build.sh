#!/bin/bash

echo "Building contract"
cargo build --release --manifest-path contract/Cargo.toml

echo "Building API"
cargo build --release
