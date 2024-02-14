#!/bin/bash
set -e
cd "`dirname $0`"

mkdir -p res

cargo build --release -p contract --target wasm32-unknown-unknown
cargo build --release -p test-token --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/*.wasm ./res/

wasm-opt -O4 res/contract.wasm -o res/contract.wasm --strip-debug --vacuum
wasm-opt -O4 res/test_token.wasm -o res/test_token.wasm --strip-debug --vacuum
