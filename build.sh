#!/bin/bash

set -e

# install build tools
if [[ -z $(which rustup) ]] ; then
	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly 
	source $HOME/.cargo/env
fi

# install wasm32 target
[[ -z $(rustup target list | grep installed | grep wasm32) ]] && rustup target add wasm32-unknown-unknown

[[ -z $(which ontio-wasm-build) ]] && cargo install --git=https://github.com/ontio/ontio-wasm-build

# clean origial build 
rm ./target/wasm32-unknown-unknown/release/*.wasm

# clean origial build 
RUSTFLAGS="-C link-arg=-zstack-size=32768" cargo build --release --target wasm32-unknown-unknown

for wasm in ./target/wasm32-unknown-unknown/release/*.wasm ; do
	ontio-wasm-build $wasm $wasm
done
