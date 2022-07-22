#!/bin/bash
set -e
cargo install wasm-pack sfz
sudo npm i wasm-opt -g
wasm-pack build --target web
# wasm-pack build --target nodejs
wasm-opt -Oz pkg/rafael_bg.wasm -o pkg/rafael_bg.wasm # execute default optimization, passes, super-focusing on code
wasm-opt -Oz ../target/wasm32-unknown-unknown/release/rafael.wasm -o PaaS/src/runtime/pkg/rafael_bg.wasm
# node index.js
# sfz -r 