use crate::vec::Ray;
use crate::vis::{Visible, Visibility};
use crate::octree::{Bounded, Bounds};
use num::Float;

// This file purely exists because of the stuff that Rust won't do easily.
// Hopefully it can be deleted eventually.

pub trait Renderable<'m, D: Float>: Visible<'m, D> + Bounded<D> {}
impl<'m, D: Float, T> Renderable<'m, D> for T where T: Visible<'m, D> + Bounded<D> {}
impl<'m, D: Float> Bounded<D> for Box<dyn Renderable<'m, D> + '_> {
  fn bounds(&self) -> Bounds<D> { self.as_ref().bounds() }
}
impl<'m, D: Float> Visible<'m, D> for Box<dyn Renderable<'m, D> + '_> {
  fn hit(&self, r: &Ray<D>) -> Option<Visibility<'m, D>> { self.as_ref().hit(r) }
}

impl<'a, T, V: Visible<'a, T>> Visible<'a, T> for Vec<V>
where
  T: num::Float,
{
  fn hit(&self, r: &Ray<T>) -> Option<Visibility<'a, T>> {
    let mut curr_bound = T::zero()..T::infinity();
    self.iter().fold(None, |nearest, item| {
      match item.hit_bounded(r, curr_bound.clone()) {
        None => nearest,
        Some(hit) => match nearest {
          None => Some(hit),
          Some(prev) if hit.param > prev.param => Some(prev),
          Some(_) => {
            curr_bound.end = hit.param;
            Some(hit)
          },
        },
      }
    })
  }
}
