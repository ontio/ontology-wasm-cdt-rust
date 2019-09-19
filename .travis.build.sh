#!/bin/bash
set -e
set -x

RUSTFLAGS="-C link-arg=-zstack-size=32768" cargo build --release --target wasm32-unknown-unknown


root_dir=$(pwd)

cd ontio-std
cargo test --features=mock

cd ../ontio-codegen
cargo test --features=mock
cd $root_dir

for dir in ./examples/* ; do
 cd $dir
 cargo test --features=mock;
 cd $root_dir
done

