#!/bin/bash
set -e
set -x

#	cargo clippy --all -- -Dwarnings;
echo 'Skipping clippy';

RUSTFLAGS="-Ctarget-cpu=mvp -C link-arg=-zstack-size=32768" cargo build -Zbuild-std=panic_abort,core,alloc --release --target wasm32-unknown-unknown

root_dir=$(pwd)

cd ontio-std
cargo test --features=mock

cd ../ontio-codegen
cargo test 
cd $root_dir

for dir in ./examples/* ; do
 cd $dir
 cargo test --features=mock;
 cd $root_dir
done

