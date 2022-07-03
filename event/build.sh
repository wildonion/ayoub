#!/bin/bash
set -e 
export WASM_NAME=event.wasm
export OPTIMIZE_WASM_NAME=event.opt.wasm
RUSTFLAGS='-C link-args=-s' rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm out/$WASM_NAME
wasm-opt -Os -o out/$WASM_NAME out/$OPTIMIZE_WASM_NAME
rm out/$WASM_NAME && ls out -lh