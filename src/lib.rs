#[cfg(not(feature = "wasi-decoder"))]
mod sandbox;
#[cfg(not(feature = "wasi-decoder"))]
pub use sandbox::*;

use serde::{Deserialize, Serialize};

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
  pub only_header: bool,
  /// JP2/J2K compressed image data.
  pub data: Vec<u8>,
}

impl DecodeImageRequest {
  pub fn new(data: Vec<u8>) -> Self {
    Self {
      data,
      only_header: false,
      params: None,
    }
  }

  pub fn new_with(data: Vec<u8>, params: DecodeParameters) -> Self {
    Self {
      data,
      only_header: false,
      params: Some(params),
    }
  }

  pub fn only_header(&self) -> Self {
    let mut header_request = self.clone();
    header_request.only_header = true;
    header_request
  }

  #[cfg(feature = "jpeg2k")]
  pub fn params(&self) -> jpeg2k::DecodeParameters {
    match self.params {
      Some(params) => params.into(),
      None => jpeg2k::DecodeParameters::new(),
    }
  }
}

/// Image pixel format.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ImageFormat {
  L8,
  La8,
  Rgb8,
  Rgba8,
  L16,
  La16,
  Rgb16,
  Rgba16,
}

#[cfg(feature = "jpeg2k")]
impl From<jpeg2k::ImageFormat> for ImageFormat {
  fn from(format: jpeg2k::ImageFormat) -> Self {
    use jpeg2k::ImageFormat::*;
    match format {
      L8 => Self::L8,
      La8 => Self::La8,
      Rgb8 => Self::Rgb8,
      Rgba8 => Self::Rgba8,
      L16 => Self::L16,
      La16 => Self::La16,
      Rgb16 => Self::Rgb16,
      Rgba16 => Self::Rgba16,
    }
  }
}

/// Image pixel data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImagePixelData {
  L8(Vec<u8>),
  La8(Vec<u8>),
  Rgb8(Vec<u8>),
  Rgba8(Vec<u8>),
  L16(Vec<u16>),
  La16(Vec<u16>),
  Rgb16(Vec<u16>),
  Rgba16(Vec<u16>),
}

#[cfg(feature = "jpeg2k")]
impl From<jpeg2k::ImagePixelData> for ImagePixelData {
  fn from(format: jpeg2k::ImagePixelData) -> Self {
    use jpeg2k::ImagePixelData::*;
    match format {
      L8(d) => Self::L8(d),
      La8(d) => Self::La8(d),
      Rgb8(d) => Self::Rgb8(d),
      Rgba8(d) => Self::Rgba8(d),
      L16(d) => Self::L16(d),
      La16(d) => Self::La16(d),
      Rgb16(d) => Self::Rgb16(d),
      Rgba16(d) => Self::Rgba16(d),
    }
  }
}

/// Component info.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct ComponentInfo {
  pub width: u32,
  pub height: u32,
  pub precision: u32,
  pub is_alpha: bool,
  pub is_signed: bool,
}

/// J2KImage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct J2KImage {
  pub width: u32,
  pub height: u32,
  pub format: ImageFormat,
  pub data: ImagePixelData,
  pub orig_width: u32,
  pub orig_height: u32,
  pub x_offset: u32,
  pub y_offset: u32,
  pub num_components: u32,
  pub components: Vec<ComponentInfo>,
}

/// Try to convert a loaded Jpeg 2000 image into a `J2KImage`.
#[cfg(feature = "jpeg2k")]
impl TryFrom<jpeg2k::Image> for J2KImage {
  type Error = jpeg2k::error::Error;

  fn try_from(img: jpeg2k::Image) -> Result<Self, Self::Error> {
    let comps = img.components();
    let num_components = img.num_components();
    let mut has_alpha = false;
    let components = comps
      .iter()
      .map(|c| {
        let is_alpha = c.is_alpha();
        if is_alpha {
          has_alpha = true;
        }
        ComponentInfo {
          width: c.width(),
          height: c.height(),
          precision: c.precision(),
          is_alpha: c.is_alpha(),
          is_signed: c.is_signed(),
        }
      })
      .collect::<Vec<_>>();
    let (format, data) = match img.get_pixels(None) {
      Ok(d) => (d.format.into(), d.data.into()),
      Err(_) => {
        let format = match (num_components, has_alpha) {
          (1, _) => ImageFormat::L8,
          (2, true) => ImageFormat::La8,
          (3, false) => ImageFormat::Rgb8,
          (4, _) => ImageFormat::Rgba8,
          _ => {
            return Err(jpeg2k::error::Error::UnsupportedComponentsError(
              num_components,
            ));
          }
        };
        (format, ImagePixelData::L8(vec![]))
      }
    };
    Ok(J2KImage {
      width: img.width(),
      height: img.height(),
      format,
      data,
      orig_width: img.orig_width(),
      orig_height: img.orig_height(),
      x_offset: img.x_offset(),
      y_offset: img.y_offset(),
      num_components,
      components,
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
      data,
      ..
    } = img;
    match data {
      crate::ImagePixelData::L8(data) => {
        let gray = GrayImage::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageLuma8(gray))
      }
      crate::ImagePixelData::La8(data) => {
        let gray = GrayAlphaImage::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageLumaA8(gray))
      }
      crate::ImagePixelData::Rgb8(data) => {
        let rgb = RgbImage::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageRgb8(rgb))
      }
      crate::ImagePixelData::Rgba8(data) => {
        let rgba = RgbaImage::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageRgba8(rgba))
      }
      crate::ImagePixelData::L16(data) => {
        let gray = ImageBuffer::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageLuma16(gray))
      }
      crate::ImagePixelData::La16(data) => {
        let gray = ImageBuffer::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageLumaA16(gray))
      }
      crate::ImagePixelData::Rgb16(data) => {
        let rgb = ImageBuffer::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageRgb16(rgb))
      }
      crate::ImagePixelData::Rgba16(data) => {
        let rgba = ImageBuffer::from_vec(width, height, data)
          .expect("Shouldn't happen.  Report to jpeg2k if you see this.");

        Ok(DynamicImage::ImageRgba16(rgba))
      }
    }
  }
}
