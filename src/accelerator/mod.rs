pub mod naive;
// TODO enable this when actually needed
// pub mod octree;

use crate::{interaction::SurfaceInteraction, shapes::Shapes};
use quick_maths::Ray;
use std::fmt::Debug;

pub trait Accelerator: Debug {
  /// Compose an accelerator from an iteration of shapes
  fn build(i: impl Iterator<Item = Shapes>) -> Self;
  /// Intersects this accelerator with a ray, returning an interaction if there was a hit.
  fn intersect_ray(&self, r: &Ray) -> Option<(SurfaceInteraction, &Shapes)>;
}
