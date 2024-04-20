#!/bin/sh

cargo build --target wasm32-wasi --profile wasm32-wasi \
	--no-default-features --features wasi-decoder,openjp2 \
	--bin wasi-decoder

cp ./target/wasm32-wasi/wasm32-wasi/wasi-decoder.wasm ./src/wasi-decoder-openjp2.wasm
