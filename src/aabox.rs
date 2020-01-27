use crate::{
  bounds::{Bounded, Bounds},
  material::Mat,
  vec::Ray,
  vis::{Visibility, Visible},
};
use num::Float;

/// Axis Aligned Bounding Box
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AABox<'m, D> {
  bounds: Bounds<D>,
  mat: &'m Mat<D>,
}

impl<D: Clone> Bounded<D> for AABox<'_, D> {
  fn bounds(&self) -> Bounds<D> { self.bounds.clone() }
}

impl<'m, D: Float> AABox<'m, D> {
  pub fn new(bounds: Bounds<D>, mat: &'m Mat<D>) -> Self { Self { bounds, mat } }
  pub fn contains(&self, r: &Ray<D>) -> bool { self.bounds.intersects_ray(&r) }
}

impl<'m, D: Float> Visible<'m, D> for AABox<'m, D> {
  fn hit(&self, r: &Ray<D>) -> Option<Visibility<'m, D>> {
    self.bounds.intersects_ray_params(&r).map(|(param, norm)| {
      let pos = r.at(param);
      Visibility {
        param,
        pos,
        norm,
        mat: self.mat,
      }
    })
  }
}
