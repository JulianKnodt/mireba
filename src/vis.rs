extern crate num;
extern crate rand;
extern crate rand_distr;
use crate::{
  bounds::Bounded,
  indexed_triangles::IndexedTriangles,
  material::{Mat, Material},
  octree::Octree,
  vec::{Ray, Vec3},
};
use num::{One, Zero};
use rand::prelude::*;
use rand_distr::{Standard, StandardNormal};
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Visibility<'m, T> {
  /// Parameter of incoming ray that this hit on
  pub(crate) param: T,
  /// Coordinate in world space of hit
  pub(crate) pos: Vec3<T>,
  /// Normal to surface of hit
  pub(crate) norm: Vec3<T>,
  /// Material of hit
  pub(crate) mat: &'m Mat<T>,
}

pub trait Visible<'m, T: num::Float> {
  // returns parameter T, position, and normal
  fn hit(&self, r: &Ray<T>) -> Option<Visibility<'m, T>>;
  fn hit_bounded(&self, r: &Ray<T>, bounds: Range<T>) -> Option<Visibility<'m, T>> {
    self.hit(r).filter(|vis| bounds.contains(&vis.param))
  }
}

// Checks whether a ray hits the entire set of indexed triangle
impl<'m, T> Visible<'m, T> for IndexedTriangles<'m, T>
where
  T: num::Float,
{
  fn hit<'a>(&'a self, r: &Ray<T>) -> Option<Visibility<'m, T>> {
    let mut curr_bound = T::zero()..T::infinity();
    self.iter().fold(None, |nearest, next| {
      next
        .hit_bounded(r, curr_bound.clone())
        .and_then(|h| match nearest {
          None => Some(h),
          Some(prev) if h.param > prev.param => Some(prev),
          Some(_) => {
            curr_bound.end = h.param;
            Some(h)
          },
        })
        .or(nearest)
    })
  }
}

impl<'m, D: num::Float, T: Bounded<D> + Visible<'m, D>> Visible<'m, D> for Octree<D, T> {
  fn hit(&self, r: &Ray<D>) -> Option<Visibility<'m, D>> {
    let mut curr_bound = D::zero()..D::infinity();
    self.intersecting_elements(*r).fold(None, |nearest, next| {
      next
        .hit_bounded(r, curr_bound.clone())
        .and_then(|h| match nearest {
          None => Some(h),
          Some(prev) if h.param > prev.param => Some(prev),
          Some(_) => {
            curr_bound.end = h.param;
            Some(h)
          },
        })
        .or(nearest)
    })
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Camera<T> {
  pos: Vec3<T>,

  // Screen positions
  ll_corner: Vec3<T>,
  hori: Vec3<T>,
  vert: Vec3<T>,
}

pub fn rand_in_unit_disk<T>() -> (T, T)
where
  T: num::Float,
  Standard: Distribution<T>, {
  let mut rng = thread_rng();
  let r = rng.gen().sqrt();
  let theta = rng.gen() * T::from(2.0 * std::f64::consts::PI).unwrap();
  (r * theta.cos(), r * theta.sin())
}

impl<T> Camera<T>
where
  T: num::Float,
  Standard: Distribution<T>,
{
  pub fn new(vert_fov_deg: T, aspect_ratio: T) -> Self {
    let theta = vert_fov_deg.to_radians();
    let half_height = (theta / T::from(2.0).unwrap()).tan();
    let half_width = half_height * aspect_ratio;
    Camera {
      ll_corner: Vec3(-half_width, -half_height, -T::one()),
      vert: Vec3(half_width * T::from(2.0).unwrap(), T::zero(), T::zero()),
      hori: Vec3(T::zero(), half_height * T::from(2.0).unwrap(), T::zero()),
      pos: Vec3::zero(),
    }
  }
  pub fn aimed(from: Vec3<T>, at: Vec3<T>, up: Vec3<T>, vert_fov_deg: T, aspect: T) -> Self {
    let theta = vert_fov_deg.to_radians();
    let half_height = (theta / T::from(2.0).unwrap()).tan();
    let half_width = half_height * aspect;
    let w = (from - at).norm();
    let u = up.cross(&w).norm();
    let v = w.cross(&u).norm();
    Camera {
      ll_corner: from - v * half_height - u * half_width - w,
      vert: u * half_width * T::from(2.0).unwrap(),
      hori: v * half_height * T::from(2.0).unwrap(),
      pos: from,
    }
  }
  pub fn to(&self, u: T, v: T) -> Ray<T> {
    Ray::new(
      self.pos,
      self.ll_corner + self.hori * v + self.vert * u - self.pos,
    )
  }
  pub fn rays(&self, n: usize, x: T, y: T, w: T, h: T) -> impl Iterator<Item = Ray<T>> + '_ {
    let mut rng = thread_rng();
    (0..n).map(move |_| self.to((x + rng.gen()) / w, (y + rng.gen()) / h))
  }
}

pub fn color<'a, V, T: 'a>(r: &Ray<T>, item: &V) -> Vec3<T>
where
  T: num::Float,
  StandardNormal: Distribution<T>,
  Standard: Distribution<T>,
  V: Visible<'a, T>, {
  if let Some(mut vis) = item.hit_bounded(&r, T::from(0.001).unwrap()..T::infinity()) {
    // return vis.norm.norm()
    vis.pos = vis.pos + vis.norm * T::from(0.00001).unwrap();
    vis
      .mat
      .scatter(r, &vis)
      .map(|(atten, bounce)| color(&bounce, item) * atten)
      .unwrap_or_else(Vec3::one)
  } else {
    Vec3::lerp(
      (r.dir.norm().1 + T::one()) * T::from(0.5).unwrap(),
      Vec3::one(),
      Vec3(T::from(0.5).unwrap(), T::from(0.7).unwrap(), T::one()),
    )
  }
}
