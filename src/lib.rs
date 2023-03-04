#[cfg(not(feature = "wasi-decoder"))]
mod sandbox;
#[cfg(not(feature = "wasi-decoder"))]
pub use sandbox::*;
