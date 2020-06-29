use super::Texture;
use crate::spectrum::Spectrum;
use quick_maths::Vec2;

#[derive(Debug)]
pub struct Constant {
  s: Spectrum,
}

impl Texture for Constant {
  fn sample(&self, _uv: Vec2) -> Spectrum { self.s }
}
