#!/bin/bash
set -e

echo "Building contracts"
cd contract
RUSTFLAGS='-C link-arg=-s' cargo +stable build --release --target wasm32-unknown-unknown
cd ..
cd example-integration
RUSTFLAGS='-C link-arg=-s' cargo +stable build --release --target wasm32-unknown-unknown
cd ..

echo "Building API"
cargo build --release
