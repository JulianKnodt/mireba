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
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Renderable<D: Float> {
  // TODo should these references so that it can save space?
  Sphere(Sphere<D>),
  AABox(AABox<D>),
  Plane(Plane<D>),
  IndexedTriangles(IndexedTriangles<D>),
}

impl<D: Float> Visible<D> for Renderable<D> {
  fn hit(&self, r: &Ray<D>) -> Option<Visibility<D>> {
    match self {
      Renderable::Sphere(ref sphere) => sphere.hit(&r),
      Renderable::AABox(ref aab) => aab.hit(&r),
      Renderable::Plane(ref p) => p.hit(&r),
      Renderable::IndexedTriangles(ref it) => it.hit(&r),
    }
  }
}

impl<D: Float> Bounded<D> for Renderable<D> {
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

impl<D: Float, V: Visible<D>> Visible<D> for Vec<V> {
  fn hit(&self, r: &Ray<D>) -> Option<Visibility<D>> {
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

macro_rules! rend_from {
  ($name: ty, $out: path) => {
    impl<T: Float> From<$name> for Renderable<T> {
      fn from(src: $name) -> Self { $out(src) }
    }
  };
}

rend_from!(Sphere<T>, Renderable::Sphere);
rend_from!(Plane<T>, Renderable::Plane);
