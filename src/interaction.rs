use num::Zero;
use quick_maths::{Vec2, Vec3};
use std::cmp::Ordering;

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
  pub fn at(t: f32, p: Vec3) -> Self { Self { t, p } }
  pub fn closer(&self, o: &Self) -> Ordering { self.t.partial_cmp(&o.t).unwrap() }
}

#[derive(Debug)]
pub struct SurfaceInteraction {
  /// Interaction for this surface interaction
  pub it: Interaction,
  /// Normal of this surface interaction
  pub normal: Vec3,

  /// UV position on this surface
  pub uv: Vec2,
  /// Incoming direction of incident light
  pub wi: Vec3,
}

#[derive(Debug)]
pub struct MediumInteraction {
  /// Location and position of this interaction
  pub it: Interaction,

  /// Incoming direction
  pub wi: Vec3,

  /// The length this ray travelled thru the medium
  pub optical_path_length: f32,
}
