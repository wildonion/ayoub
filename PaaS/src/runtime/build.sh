#!/bin/bash
set -e
cargo install wasm-pack --force
sudo npm i wasm-opt -g
wasm-pack build --target bundler --out-dir bundlerPKG
wasm-pack build --target web --out-dir webPKG
wasm-pack build --target nodejs --out-dir nodePKG
wasm-opt -Oz pkg/rafael_bg.wasm -o pkg/rafael_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz ../target/wasm32-unknown-unknown/release/rafael.wasm -o PaaS/src/runtime/rafael.wasm