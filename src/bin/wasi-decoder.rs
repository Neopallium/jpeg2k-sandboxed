use anyhow::Result;
use std::io::Read;

use jpeg2k::*;

fn main() -> Result<()> {
  let mut data = Vec::new();
  std::io::stdin().read_to_end(&mut data)?;

  let jp2_image = Image::from_bytes(&data).unwrap();
  eprintln!("dump image: {:#?}", jp2_image);
  let image = jp2_image.get_pixels(None).unwrap();

  let mut stdout = std::io::stdout();
  rmp_serde::encode::write(&mut stdout, &image)?;
  Ok(())
}
