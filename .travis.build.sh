#!/bin/bash
set -e
set -x

cd ontio-std
cargo build
cargo test --features=mock

cd ../ontio-codegen
cargo build
cargo test --features=mock

cd ../examples/token-codegen
cargo build
cargo test --features=mock
