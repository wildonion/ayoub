#!/bin/bash
set -e 
export WASM_NAME=main.wasm
RUSTFLAGS='-C link-args=-s' rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm out/$WASM_NAME
wasm-opt -Os -o out/output_s.wasm out/$WASM_NAME
rm out/output_s.wasm && ls out -lh