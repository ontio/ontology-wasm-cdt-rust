#!/bin/bash
set -e
set -x

if rustup component add clippy;
then
	cargo clippy --all -- -Dwarnings;
else
	echo 'Skipping clippy';
fi

cargo build

cd ontio-std
cargo test --features=mock

cd ../ontio-codegen
cargo test --features=mock

cd ../examples/token-codegen
cargo test --features=mock
