use crate::{
  bounds::{Bounded, Bounds},
  vec::Ray,
  vis::{Visibility, Visible},
};
use num::Float;
use serde::{Deserialize, Serialize};

/// Axis Aligned Bounding Box
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AABox<D> {
  bounds: Bounds<D>,
}

impl<D: Clone> Bounded<D> for AABox<D> {
  fn bounds(&self) -> Bounds<D> { self.bounds.clone() }
}

impl<D: Float> AABox<D> {
  pub fn new(bounds: Bounds<D>) -> Self { Self { bounds } }
  pub fn contains(&self, r: &Ray<D>) -> bool { self.bounds.intersects_ray(&r) }
}

impl<D: Float> Visible<D> for AABox<D> {
  fn hit(&self, r: &Ray<D>) -> Option<Visibility<D>> {
    self.bounds.intersects_ray_params(&r).map(|(param, norm)| {
      let pos = r.at(param);
      Visibility { param, pos, norm }
    })
  }
}
