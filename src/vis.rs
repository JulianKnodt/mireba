extern crate num;
extern crate rand;
extern crate rand_distr;
use crate::{
  indexed_triangles::IndexedTriangles,
  light::Light,
  material::{Mat, Material},
  object::Object,
  vec::{Ray, Vec3, Vector},
};
use num::{traits::float::FloatConst, Float, Zero};
use rand::prelude::*;
use rand_distr::{Standard, StandardNormal};
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Visibility<T> {
  /// Parameter of incoming ray that this hit on
  pub param: T,
  /// Coordinate in world space of hit
  pub pos: Vec3<T>,
  /// Normal to surface of hit
  pub norm: Vec3<T>,
}

pub trait Visible<T: Float> {
  // returns parameter T, position, and normal
  /// Returns the visibility of this object from a given ray
  fn hit(&self, r: &Ray<T>) -> Option<Visibility<T>>;
  /// Whether or not this object is intersected in a certain range
  fn hit_bounded(&self, r: &Ray<T>, bounds: Range<T>) -> Option<Visibility<T>> {
    self.hit(r).filter(|vis| bounds.contains(&vis.param))
  }
  /// Intended to indicate whether this object is on the path of a ray.
  /// Can be optimized to not compute normals or intersection location.
  // TODO might need to revisit to see if the lifetime bound on the ray is ok
  fn on_path(&self, r: &Ray<T>) -> bool { self.hit(r).is_some() }
}

// Checks whether a ray hits the entire set of indexed triangle
impl<T: Float> Visible<T> for IndexedTriangles<T> {
  fn hit(&self, r: &Ray<T>) -> Option<Visibility<T>> {
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
  T: Float + FloatConst,
  Standard: Distribution<T>, {
  let mut rng = thread_rng();
  let r = rng.gen().sqrt();
  let theta = rng.gen() * T::from(2.0).unwrap() * T::PI();
  (r * theta.cos(), r * theta.sin())
}

impl<T> Camera<T>
where
  T: Float,
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

/// Returns the color for a given ray and some visible thing
pub fn color<'i, 'a, V: 'i, T: 'a + 'i, I>(
  r: &Ray<T>,
  items: I,
  depth_left: usize,
  lights: &[Light<T>],
) -> Vec3<T>
where
  'a: 'i,
  T: Float,
  I: Iterator<Item = &'i Object<&'a Mat<T>, V>> + Clone,
  StandardNormal: Distribution<T>,
  Standard: Distribution<T>,
  V: Visible<T>, {
  let bg = Vec3::from(T::from(0.05).unwrap());
  let eps = T::from(1e-4).unwrap(); // T::epsilon(); // ?
  let initial = (eps..T::infinity(), None);
  let (_, hit) = items.clone().fold(
    initial,
    |(mut range, prev): (Range<T>, Option<(Visibility<T>, _)>), item| {
      let hit = item.shape.hit_bounded(r, range.clone());
      let hit = match (prev, hit) {
        (v, None) => v,
        (Some((h, p)), Some(n)) if h.param < n.param => Some((h, p)),
        (None, Some(n)) | (Some(_), Some(n)) => {
          range.end = n.param;
          Some((n, item))
        },
      };
      (range, hit)
    },
  );
  match hit {
    None => bg,
    Some((mut vis, item)) => {
      // return vis.norm.norm(); // DEBUG
      vis.pos = vis.pos + vis.norm * (eps + eps);
      let out_color = lights.iter().fold(Vec3::zero(), |acc: Vec3<T>, l| {
        acc + item.mat.color(&r, &vis, &l)
      });
      let albedo = item.mat.albedo();
      let recur = if depth_left == 0 || albedo.is_zero() {
        Vec3::zero()
      } else {
        item
          .mat
          .reflected(r, &vis)
          .map(|bounce| color(&bounce, items, depth_left - 1, lights) * albedo)
          .unwrap_or_else(Vec3::zero)
      };
      out_color + recur
    },
  }
}
