#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/ontio_std.wasm  wasm_demo.wasm
./wasm-prune -e add_one,add wasm_demo.wasm  wasm_demo_prune.wasm
wasm2wat wasm_demo.wasm  -o wasm_demo.wast
wasm2wat wasm_demo_prune.wasm  -o wasm_demo_prune.wast

