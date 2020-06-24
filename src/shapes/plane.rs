use linalg::num::Float;
use linalg::vec::{Ray, Vec3, Vector};
use crate::{
  vis::{Visibility, Visible},
};

/// Represents a plane in 3d space
/// Defined by the equation (P * NormaL) + w = 0
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Plane<D> {
  /// Normal to the plane
  normal: Vec3<D>,
  /// Offset of the plane from the origin
  w: D,
}

impl<D: Float> Plane<D> {
  pub fn new(normal: Vec3<D>, w: D) -> Self {
    let normal = normal.norm();
    Self { normal, w }
  }
  /// Returns a representative point on this plane
  pub fn repr_point(&self) -> Vec3<D> { -(self.normal * self.w) / self.normal.sqr_magn() }
  #[inline]
  pub fn on_plane(&self, v: &Vec3<D>) -> bool {
    self.normal.dot(v) + self.w < D::from(0.001).unwrap()
  }
}

impl<D: Float> Visible<D> for Plane<D> {
  fn hit(&self, r: &Ray<D>) -> Option<Visibility<D>> {
    let d = self.normal.dot(&r.dir);
    if d.is_zero() {
      return None;
    }
    let param = -(r.pos.dot(&self.normal) + self.w) / d;
    if param.is_sign_negative() {
      return None;
    }
    let pos = r.at(param);
    Some(Visibility {
      pos,
      param,
      norm: self.normal,
    })
  }
}

#[cfg(test)]
mod test_plane {
  use super::Plane;
  use crate::{vec::Ray, vis::Visible};
  use quickcheck::TestResult;
  quickcheck! {
    fn repr_point_on_plane(plane: Plane<f32>) -> bool {
      plane.on_plane(&plane.repr_point())
    }
  }
  quickcheck! {
    // Tests that any ray not parallel to the plane hits it
    fn hits_plane(r: Ray<f32>, plane: Plane<f32>) -> TestResult {
      match plane.hit(&r) {
        None => TestResult::discard(),
        Some(vis) => TestResult::from_bool(plane.on_plane(&vis.pos)),
      }
    }
  }
}
