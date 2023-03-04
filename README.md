# jpeg2k-sandboxed

A sandboxed Jpeg2k image decoder.  The jpeg2k decoder (uses `openjpeg-sys`) is compiled to a WASI module
and sandboxed using `wasmtime`.

## Build

The `./src/wasi-decoder.wasm` file can be rebuild by running `./rebuild-wasi-decoder.sh`.

Requires [wasi-sdk](https://github.com/WebAssembly/wasi-sdk).

## Example: Convert a Jpeg 2000 image to a png image.

```rust
use jpeg2k_sandboxed::*;

fn main() {
	// The decoder object can be shared across threads.
  let decoder = Jpeg2kSandboxed::new().expect("Failed to load decoder");

  let mut file = File::open("./assets/example.j2k").expect("Failed to open file.");
  let mut buf = Vec::new();
  file.read_to_end(&mut buf).expect("Failed to read file.");

  // Request decoding of image from bytes.
  let req = DecodeImageRequest::new(buf);
  let image = decoder.decode(&req).expect("Decode failed.");

  // Convert to a `image::DynamicImage`
  let img: image::DynamicImage = image.try_into()?;

  // Save as png file.
  img.save("out.png")?;
}
```
