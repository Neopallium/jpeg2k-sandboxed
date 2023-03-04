#!/bin/sh
WASI_SDK_PATH="/opt/wasi-sdk"

CC="${WASI_SDK_PATH}/bin/clang -D_WASI_EMULATED_PROCESS_CLOCKS -lwasi-emulated-process-clocks --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot" \
	cargo build --target wasm32-wasi --profile wasm32-wasi \
	--no-default-features --features wasi-decoder \
	--bin wasi-decoder

cp ./target/wasm32-wasi/wasm32-wasi/wasi-decoder.wasm ./src/
