use std::env;
use std::fs::File;
use std::io::Read;
use anyhow::Result;

use jpeg2k_sandboxed::*;

fn main() -> Result<()> {
  let jp2_filename = env::args().nth(1).unwrap_or_else(|| "test.j2k".to_string());
  let reduce = env::args()
    .nth(2)
    .unwrap_or_else(|| "0".to_string())
    .parse::<u32>()
    .expect("Reduce must be an integer.");
  let layers = env::args()
    .nth(3)
    .unwrap_or_else(|| "0".to_string())
    .parse::<u32>()
    .expect("Layers must be an integer.");

  // Decode parameters.
  let params = DecodeParameters {
    reduce,
    layers,
    strict: false,
    area: None,
  };

  eprintln!("Read file: {jp2_filename}");
  let mut file = File::open(jp2_filename)?;
  let mut buf = Vec::new();
  file.read_to_end(&mut buf)?;
  let mut req = DecodeImageRequest::new_with(buf, params);
  // Only decode the image's header.
  req.only_header = true;

  let decoder = Jpeg2kSandboxed::new()?;
  let img = decoder.decode(&req).expect("Image read header");

  println!("J2k image: {img:#?}");

  Ok(())
}
