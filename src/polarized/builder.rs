use crate::spectrum::{Luminance, Spectrum, RGB};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Builder {
  Monochrome(Luminance),
  RGB(RGB),
  Wavelength(f32),
  // TODO more here
}

impl From<Builder> for Spectrum {
  fn from(b: Builder) -> Self {
    use Builder::*;
    cfg_if::cfg_if! {
      if #[cfg(feature="mono")] {
        match b {
          Monochrome(l) => l,
          RGB(rgb) => super::srgb_to_gray(rgb),
          Wavelength(_w) => todo!()
        }
      } else {
        match b {
          Monochrome(l) => crate::spectrum::RGB::of(l),
          RGB(rgb) => rgb,
          Wavelength(_w) => todo!()
        }
      }
    }
  }
}
