# jpeg2k-sandboxed

A sandboxed Jpeg2k image decoder.  The jpeg2k decoder (uses `openjpeg-sys`) is compiled to a WASI module
and sandboxed using `wasmtime`.

## Build

The `./src/wasi-decoder.wasm` file can be rebuild by running `./rebuild-wasi-decoder.sh`.

Requires [wasi-sdk](https://github.com/WebAssembly/wasi-sdk).

