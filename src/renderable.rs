use crate::{
  aabox::AABox,
  bounds::{Bounded, Bounds},
  indexed_triangles::IndexedTriangles,
  plane::Plane,
  sphere::Sphere,
  vec::Ray,
  vis::{Visibility, Visible},
};
use num::Float;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Renderable<'m, D> {
  Sphere(Sphere<'m, D>),
  AABox(AABox<'m, D>),
  Plane(Plane<'m, D>),
  IndexedTriangles(IndexedTriangles<'m, D>),
}

impl<'m, D: Float> Visible<'m, D> for Renderable<'m, D> {
  fn hit(&self, r: &Ray<D>) -> Option<Visibility<'m, D>> {
    match self {
      Renderable::Sphere(ref sphere) => sphere.hit(&r),
      Renderable::AABox(ref aab) => aab.hit(&r),
      Renderable::Plane(ref p) => p.hit(&r),
      Renderable::IndexedTriangles(ref it) => it.hit(&r),
    }
  }
}

impl<'m, D: Float> Bounded<D> for Renderable<'m, D> {
  fn bounds(&self) -> Bounds<D> {
    match self {
      Renderable::Sphere(ref sphere) => sphere.bounds(),
      Renderable::AABox(ref aab) => aab.bounds(),
      Renderable::IndexedTriangles(ref _it) => todo!(),

      // These things are unbounded so just panic if they ever get called
      Renderable::Plane(_) => panic!("Cannot bound plane"),
    }
  }
}

impl<'a, D: Float, V: Visible<'a, D>> Visible<'a, D> for Vec<V> {
  fn hit(&self, r: &Ray<D>) -> Option<Visibility<'a, D>> {
    let mut curr_bound = D::zero()..D::infinity();
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
