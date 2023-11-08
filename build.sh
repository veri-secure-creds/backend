#!/bin/bash
set -e

echo "Building contract"
RUSTFLAGS='-C link-arg=-s' cargo build --release --manifest-path contract/Cargo.toml

echo "Building API"
cargo build --release
