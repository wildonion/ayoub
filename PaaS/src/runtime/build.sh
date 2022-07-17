#!/bin/bash
set -e
cargo install wasm-pack sfz
sudo npm i wasm-opt -g
wasm-pack build --target web
# wasm-pack build --target nodejs
wasm-opt -Oz pkg/rafael_bg.wasm -o pkg/rafael_bg.wasm # execute default optimization, passes, super-focusing on code
# node index.js
# sfz -r