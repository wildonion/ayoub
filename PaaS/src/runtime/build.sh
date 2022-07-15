#!/bin/bash
set -e
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
sudo npm i wasm-opt -g
# wasm-pack build --target web
wasm-pack build --target nodejs
wasm-opt -Oz pkg/rafael_bg.wasm -o pkg/rafael_bg.wasm # execute default optimization, passes, super-focusing on code
node index.js