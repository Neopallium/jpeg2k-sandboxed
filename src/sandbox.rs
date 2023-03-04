use anyhow::Result;

use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;

use crate::*;

pub struct Jpeg2kSandboxed {
  engine: Engine,
  instance_pre: InstancePre<WasiCtx>,
}

impl Jpeg2kSandboxed {
  pub fn new() -> Result<Self> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s: &mut WasiCtx| s)?;
  
    let wasm = include_bytes!("wasi-decoder.wasm");
    let module = Module::from_binary(&engine, wasm)?;

    log::debug!("Link Jpeg2k-component.");
    let instance_pre = linker
      .instantiate_pre(&module)?;

    Ok(Self {
      engine,
      instance_pre,
    })
  }

  pub fn decode(&self, req: &DecodeImageRequest) -> Result<J2KImage> {
    let req = rmp_serde::to_vec(&req)?;
    let stdin = ReadPipe::from(req);
    let stdout = WritePipe::new_in_memory();
  
    let wasi = WasiCtxBuilder::new()
      .stdin(Box::new(stdin.clone()))
      .stdout(Box::new(stdout.clone()))
      .inherit_env()?
      .build();
  
    let mut store = Store::new(&self.engine, wasi);
  
    log::debug!("Run Jpeg2k-component.");
    self.instance_pre.instantiate(&mut store)?
      .get_typed_func::<(), ()>(&mut store, "_start")?
      .call(&mut store, ())?;

    drop(store);
  
    let contents: Vec<u8> = stdout
      .try_into_inner()
      .map_err(|_err| anyhow::Error::msg("sole remaining reference"))?
      .into_inner();
  
    let image: Result<J2KImage, String> = rmp_serde::from_slice(&contents)?;
    Ok(image.map_err(|e| anyhow::anyhow!("{e}"))?)
  }
}
