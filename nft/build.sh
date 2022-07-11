#!/bin/bash
set -e
export WASM_NAME=nft.wasm
RUSTFLAGS='-C link-args=-s' rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
cp ../target/wasm32-unknown-unknown/release/*.wasm out/$WASM_NAME
sudo apt install binaryen wabt && cargo install wasm-snip wasm-gc
for p in "$@"; do
    w=$(basename -- $p)
    echo "Minifying $w, make sure it is not stripped"
    wasm-snip $p --snip-rust-fmt-code --snip-rust-panicking-code -p core::num::flt2dec::.* -p core::fmt::float::.*  --output temp-$w
    wasm-gc temp-$w
    wasm-strip temp-$w
    wasm-opt -Os -o out/temp-$w out/minified-$WASM_NAME
    rm temp-$w
    echo $w `stat -c "%s" $p` "bytes ->" `stat -c "%s" minified-$w` "bytes, see minified-$w"
done