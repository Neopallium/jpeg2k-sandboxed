use std::env;
use std::fs::File;
use std::io::Read;
use anyhow::Result;

use jpeg2k_sandboxed::*;

fn main() -> Result<()> {
  let jp2_filename = env::args().nth(1).unwrap_or_else(|| "test.j2k".to_string());
  eprintln!("Read file: {jp2_filename}");
  let mut file = File::open(jp2_filename)?;
  let mut buf = Vec::new();
  file.read_to_end(&mut buf)?;

  let decoder = Jpeg2kSandboxed::new()?;
  let image = decoder.decode(&buf)?;
  eprintln!("dump image: {}x{}, format: {:?}", image.width, image.height, image.format);
  Ok(())
}
