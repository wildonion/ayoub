#!/bin/bash
set -e
export WASM_NAME=market.wasm
RUSTFLAGS='-C link-args=-s' rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
cp ../target/wasm32-unknown-unknown/release/*.wasm out/$WASM_NAME
sudo npm i wasm-opt -g && sudo apt install binaryen wabt && cargo install wasm-snip wasm-gc
wasm-snip out/$WASM_NAME --snip-rust-fmt-code --snip-rust-panicking-code -p core::num::flt2dec::.* -p core::fmt::float::.*  --output out/temp-$WASM_NAME
wasm-gc out/temp-$WASM_NAME
wasm-strip out/temp-$WASM_NAME
wasm-opt -Oz out/temp-$WASM_NAME -o out/minified-$WASM_NAME # execute default optimization, passes, super-focusing on code
sudo rm out/temp-$WASM_NAME
echo $w `stat -c "%s" $p` "bytes ->" `stat -c "%s" out/minified-$WASM_NAME` "bytes, see out/minified-$WASM_NAME"