#!/bin/bash

RUSTFLAGS="-C link-arg=-zstack-size=32768" cargo build --release --target wasm32-unknown-unknown
cp ../../target/wasm32-unknown-unknown/release/token_zero_copy.wasm  wasm_demo.wasm
wasm2wat wasm_demo.wasm  -o wasm_demo.wast
wat2wasm wasm_demo.wast  -o wasm_demo_nocustom.wasm
wasm2wat wasm_demo_nocustom.wasm  -o wasm_demo_nocustom.wast
./wasm-prune -e add_one,add,boxed wasm_demo.wasm  wasm_demo_prune.wasm
wasm2wat wasm_demo_prune.wasm  -o wasm_demo_prune.wast
wat2wasm wasm_demo_prune.wast  -o wasm_demo_prune_no_custom.wasm
wasm2wat wasm_demo_prune_no_custom.wasm  -o wasm_demo_prune_no_custom.wast

