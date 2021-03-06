#[allow(unused_imports)]
use quick_maths::{Vec3, Vector};

/// RGB is just an alias for vec3
pub type RGB = Vec3;
/// Luminance is just a simple float (maybe this will change later?)
pub type Luminance = f32;

cfg_if::cfg_if! {
  if #[cfg(feature="mono")] {
    /// Spectrum type is one channel luminance in mono
    pub type Spectrum = Luminance;
    pub const fn to_rgb(s: Spectrum) -> RGB { Vector([s, s, s]) }
    pub fn from_rgb(rgb: RGB) -> Spectrum {
      let Vector([r, g, b]) = rgb;
      // TODO not correct but... close enough
      (r + g + b)/3.0
      // wonder how geometic mean would look
    }
    pub const fn from_mono(l: Luminance) -> Spectrum { l }
  } else if #[cfg(feature="polarized")] {
    todo!();
  } else {
    /// Spectrum type is three channel RGB by default
    pub type Spectrum = RGB;
    pub const fn to_rgb(s: Spectrum) -> RGB { s }
    pub const fn from_rgb(rgb: RGB) -> Spectrum { rgb }
    pub const fn from_mono(l: Luminance) -> Spectrum { Vector([l, l, l]) }
  }
  // TODO add other spectrum types here
}
