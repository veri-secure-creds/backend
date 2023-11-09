#!/bin/bash
set -e

echo "deploying registry"
cd contract
near dev-deploy ./target/wasm32-unknown-unknown/release/contract.wasm
cd ..

echo "deploying example integration"
cd example-integration
near dev-deploy ./target/wasm32-unknown-unknown/release/example_integration.wasm
cd ..
