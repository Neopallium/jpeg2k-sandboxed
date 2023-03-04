#[cfg(not(feature = "wasi-decoder"))]
mod sandbox;
#[cfg(not(feature = "wasi-decoder"))]
pub use sandbox::*;

use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DecodeArea {
  pub start_x: u32,
  pub start_y: u32,
  pub end_x: u32,
  pub end_y: u32,
}

impl std::str::FromStr for DecodeArea {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let dim = s
      .splitn(4, ":")
      .map(|v| v.parse::<u32>())
      .collect::<Result<Vec<u32>, _>>()?;
    Ok(Self {
      start_x: dim.get(0).copied().unwrap_or(0),
      start_y: dim.get(1).copied().unwrap_or(0),
      end_x: dim.get(2).copied().unwrap_or(0),
      end_y: dim.get(3).copied().unwrap_or(0),
    })
  }
}

/// DecodeParameters
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct DecodeParameters {
  pub reduce: u32,
  pub strict: bool,
  pub layers: u32,
  pub area: Option<DecodeArea>,
}

#[cfg(feature = "jpeg2k")]
impl From<DecodeArea> for jpeg2k::DecodeArea {
  fn from(p: DecodeArea) -> Self {
    Self::new(p.start_x, p.start_y, p.end_x, p.end_y)
  }
}

#[cfg(feature = "jpeg2k")]
impl From<DecodeParameters> for jpeg2k::DecodeParameters {
  fn from(p: DecodeParameters) -> Self {
    Self::new()
      .reduce(p.reduce)
      .strict(p.strict)
      .layers(p.layers)
      .decode_area(p.area.map(|a| a.into()))
  }
}

/// DecodeImageRequest
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DecodeImageRequest {
  pub params: Option<DecodeParameters>,
  /// JP2/J2K compressed image data.
  pub data: Vec<u8>,
}

impl DecodeImageRequest {
  pub fn new(data: Vec<u8>) -> Self {
    Self {
      data,
      params: None,
    }
  }

  pub fn new_with(data: Vec<u8>, params: DecodeParameters) -> Self {
    Self {
      data,
      params: Some(params),
    }
  }

  #[cfg(feature = "jpeg2k")]
  pub fn params(&self) -> jpeg2k::DecodeParameters {
    match self.params {
      Some(params) => params.into(),
      None => jpeg2k::DecodeParameters::new(),
    }
  }
}

/// Image Data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ImageFormat {
  L8,
  La8,
  Rgb8,
  Rgba8,
}

/// J2KImage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct J2KImage {
  pub width: u32,
  pub height: u32,
  pub format: ImageFormat,
  pub data: Vec<u8>,
  pub orig_width: u32,
  pub orig_height: u32,
  pub x_offset: u32,
  pub y_offset: u32,
  pub num_components: u32,
}

/// Convert a loaded Jpeg 2000 image into a `image::DynamicImage`.
#[cfg(feature = "jpeg2k")]
impl From<jpeg2k::ImageFormat> for ImageFormat {
  fn from(format: jpeg2k::ImageFormat) -> Self {
    use jpeg2k::ImageFormat::*;
    match format {
      L8 => Self::L8,
      La8 => Self::La8,
      Rgb8 => Self::Rgb8,
      Rgba8 => Self::Rgba8,
    }
  }
}

/// Try to convert a loaded Jpeg 2000 image into a `image::DynamicImage`.
#[cfg(feature = "jpeg2k")]
impl TryFrom<jpeg2k::Image> for J2KImage {
  type Error = jpeg2k::error::Error;

  fn try_from(img: jpeg2k::Image) -> Result<Self, Self::Error> {
    let d = img.get_pixels(None)?;
    Ok(J2KImage {
      width: d.width,
      height: d.height,
      format: d.format.into(),
      data: d.data,
      orig_width: img.orig_width(),
      orig_height: img.orig_height(),
      x_offset: img.x_offset(),
      y_offset: img.y_offset(),
      num_components: img.num_components(),
    })
  }
}

/// Try to convert a loaded Jpeg 2000 image into a `image::DynamicImage`.
#[cfg(feature = "image")]
impl TryFrom<J2KImage> for ::image::DynamicImage {
  type Error = ();

  fn try_from(img: J2KImage) -> Result<::image::DynamicImage, Self::Error> {
    use image::*;
    let J2KImage {
      width,
      height,
      format,
      data,
      ..
    } = img;
    match format {
      crate::ImageFormat::L8 => {
        let gray = GrayImage::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageLuma8(gray))
      }
      crate::ImageFormat::La8 => {
        let gray = GrayAlphaImage::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageLumaA8(gray))
      }
      crate::ImageFormat::Rgb8 => {
        let rgb = RgbImage::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageRgb8(rgb))
      }
      crate::ImageFormat::Rgba8 => {
        let rgba = RgbaImage::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageRgba8(rgba))
      }
    }
  }
}
