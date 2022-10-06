#!/bin/bash
set -e
cargo install wasm-pack --force
sudo npm i wasm-opt -g
wasm-pack build --target bundler --out-dir bundlerPKG
wasm-pack build --target web --out-dir webPKG
wasm-pack build --target nodejs --out-dir nodePKG
wasm-opt -Oz bundlerPKG/rafael_bg.wasm -o bundlerPKG/rafael_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz webPKG/rafael_bg.wasm -o webPKG/rafael_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz nodePKG/rafael_bg.wasm -o nodePKG/rafael_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz ../../../target/wasm32-unknown-unknown/release/rafael.wasm -o rafael.wasm



curl https://wasmtime.dev/install.sh -sSf | bash
rustup target add wasm32-wasi
cargo build --bin rafael --target wasm32-wasi --release
sudo cp ../../../target/wasm32-wasi/release/rafael.wasm ./rafael.wasm
wasmtime rafael.wasm