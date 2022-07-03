#!/bin/bash
set -e
export WASM_NAME=nft.wasm
RUSTFLAGS='-C link-args=-s' rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
cp ../target/wasm32-unknown-unknown/release/*.wasm out/$WASM_NAME
wasm-opt -Os -o out/$WASM_NAME out/$WASM_NAME