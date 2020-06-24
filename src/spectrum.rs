use quick_maths::Vec3;

cfg_if::cfg_if! {
  if #[cfg(feature="mono")] {
    pub type Spectrum = Luminance;
  } else {
    /// Spectrum type is by default RGB
    pub type Spectrum = RGB;
  }
  // TODO add other spectrum types here
}

/// RGB is just an alias for vec3
pub type RGB = Vec3;
/// Luminance is just a simple float (maybe this will change later?)
pub type Luminance = f32;

cfg_if::cfg_if! {
  if #[cfg(feature="mono")] {
    pub const fn to_rgb(s: Spectrum) -> RGB { RGB::of(s) }
  } else {
    pub const fn to_rgb(s: Spectrum) -> RGB { s }
  }
}
