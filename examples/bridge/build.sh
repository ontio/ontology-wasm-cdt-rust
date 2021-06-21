RUSTFLAGS="-C link-arg=-zstack-size=32768" cargo build --release --target wasm32-unknown-unknown
cd ../../target/wasm32-unknown-unknown/release
ontio-wasm-build bridge.wasm