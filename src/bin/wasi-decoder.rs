use anyhow::Result;

use jpeg2k::*;

use jpeg2k_sandboxed::*;

fn decode(req: DecodeImageRequest) -> Result<J2KImage> {
  let params = req.params();

  let jp2_image = Image::from_bytes_with(&req.data, params)?;
  Ok(jp2_image.try_into()?)
}

fn main() -> Result<()> {
  let req: DecodeImageRequest = rmp_serde::from_read(std::io::stdin())?;

  let res = decode(req).map_err(|e| e.to_string());
  let mut stdout = std::io::stdout();
  rmp_serde::encode::write(&mut stdout, &res)?;
  Ok(())
}
