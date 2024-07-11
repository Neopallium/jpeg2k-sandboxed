use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::Read;

use jpeg2k_sandboxed::*;

use image::DynamicImage;

fn main() -> Result<()> {
  let jp2_filename = env::args().nth(1).unwrap_or_else(|| "test.j2k".to_string());
  let savename = env::args().nth(2).unwrap_or_else(|| "test.jpg".to_string());
  let reduce = env::args()
    .nth(3)
    .unwrap_or_else(|| "0".to_string())
    .parse::<u32>()
    .expect("Reduce must be an integer.");
  let layers = env::args()
    .nth(4)
    .unwrap_or_else(|| "0".to_string())
    .parse::<u32>()
    .expect("Layers must be an integer.");
  let decode_area = env::args()
    .nth(5)
    .and_then(|area| area.parse::<DecodeArea>().ok());

  // Decode parameters.
  let params = DecodeParameters {
    reduce,
    layers,
    strict: false,
    area: decode_area,
  };

  eprintln!("Read file: {jp2_filename}");
  let mut file = File::open(jp2_filename)?;
  let mut buf = Vec::new();
  file.read_to_end(&mut buf)?;
  let req = DecodeImageRequest::new_with(buf, params);

  let decoder = Jpeg2kSandboxed::new()?;
  let image = decoder.decode(&req)?;
  eprintln!(
    "dump image: size={}x{}, orig_size={}x{}, offset={}x{}, components={}, format: {:?}",
    image.width,
    image.height,
    image.orig_width,
    image.orig_height,
    image.x_offset,
    image.y_offset,
    image.num_components,
    image.format
  );
  let img: DynamicImage = (image).try_into().expect("should convert");
  img.save(&savename)?;
  Ok(())
}
