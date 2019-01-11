#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/demo.wasm  wasm_demo.wasm
./wasm-prune -e add_one,add,boxed wasm_demo.wasm  wasm_demo_prune.wasm
wasm2wat wasm_demo.wasm  -o wasm_demo.wast
wasm2wat wasm_demo_prune.wasm  -o wasm_demo_prune.wast
wat2wasm wasm_demo_prune.wast  -o wasm_demo_prune_no_custom.wasm
wasm2wat wasm_demo_prune_no_custom.wasm  -o wasm_demo_prune_no_custom.wast

