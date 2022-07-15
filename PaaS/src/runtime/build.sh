#!/bin/bash
set -e
sudo npm i wasm-opt -g && sudo apt install binaryen wabt && cargo install wasm-snip wasm-gc
wasm-pack build --target web
wasm-pack build --target nodejs
wasm-snip pkg/rafael_bg.wasm --snip-rust-fmt-code --snip-rust-panicking-code -p core::num::flt2dec::.* -p core::fmt::float::.*  --output pkg/temp-rafael_bg.wasm
wasm-gc pkg/temp-rafael_bg.wasm
wasm-strip pkg/temp-rafael_bg.wasm
wasm-opt -Oz pkg/temp-rafael_bg.wasm -o pkg/rafael_bg.wasm # execute default optimization, passes, super-focusing on code
sudo rm pkg/temp-rafael_bg.wasm
echo pkg/rafael_bg.wasm `stat -c "%s" pkg/rafael_bg.wasm` "bytes ->" `stat -c "%s" pkg/rafael_bg.wasm` "bytes, see pkg/rafael_bg.wasm"
node index.js