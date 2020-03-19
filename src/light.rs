use crate::{
  vec::Vec3,
  color::Color,
}

/// Represents a point light source while rendering
#[derive(Debug, Clone)]
pub struct PointLight<T=f32> {
  /// Position of this light
  pub pos: Vec3<T>,
  /// Intensity in range 0-1
  pub intensity: T,

  pub color: Color<T>,
}
