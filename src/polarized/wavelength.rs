use crate::spectrum::RGB;

pub const MAX_WAVELENGTH: f32 = 830.0;
pub const MIN_WAVELENGTH: f32 = 380.0;

#[derive(Debug, PartialEq)]
pub struct Wavelength(f32);

impl Wavelength {
  pub fn ro_rgb(self) -> RGB {
    
  }
}
