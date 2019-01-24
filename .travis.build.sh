#!/bin/bash
set -e
set -x

cargo build

cd ontio-std
cargo test --features=mock

cd ../ontio-codegen
cargo test --features=mock

cd ../examples/token-codegen
cargo test --features=mock
