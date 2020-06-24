use num::Zero;
use quick_maths::{Vec2, Vec3};

#[derive(Debug)]
pub struct Interaction {
  /// Parameter along this ray that produced this interaction
  pub t: f32,
  /// Position in world-space of this interaction
  pub p: Vec3,
}

impl Default for Interaction {
  fn default() -> Self {
    Interaction {
      t: f32::INFINITY,
      p: Vec3::zero(),
    }
  }
}

impl Interaction {
  pub fn new() -> Self { Interaction::default() }
}

#[derive(Debug)]
pub struct SurfaceInteraction {
  pub it: Interaction,
  pub normal: Vec3,
  pub uv: Vec2,
  /// Incoming direction of incident light
  pub wi: Vec3,
}
