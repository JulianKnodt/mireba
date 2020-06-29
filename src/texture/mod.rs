pub mod bitmap;
pub mod constant;

use crate::spectrum::Spectrum;
use quick_maths::Vec2;

pub trait Texture: std::fmt::Debug {
  fn sample(&self, uv: Vec2) -> Spectrum;
}
