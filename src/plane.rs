use crate::{
  vec::{Ray, Vec3, Vector},
  vis::{Visibility, Visible},
};
use num::Float;
use serde::{Deserialize, Serialize};

/// Represents a plane in 3d space
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plane<D> {
  /// Normal to the plane
  normal: Vec3<D>,
  /// Offset of the plane from the origin
  w: D,
}

impl<D: Float> Plane<D> {
  pub fn new(normal: Vec3<D>, w: D) -> Self { Self { normal, w } }
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

// TODO fix flaky test
#[cfg(test)]
mod test_plane {
  use super::Plane;
  use crate::{vec::Ray, vis::Visible};
  use quickcheck::TestResult;
  quickcheck! {
    // Tests that any ray not parallel to the plane hits it
    fn hits_plane(r: Ray<f32>, plane: Plane<'static, f32>) -> TestResult {
      let pos_d = r.pos.dot(&plane.normal);
      let dir_d = r.dir.dot(&plane.normal);
      if pos_d.signum() ==  dir_d.signum() || dir_d == 0.0 { return TestResult::discard() }
      TestResult::from_bool(plane.hit(&r).is_some())
    }
  }
}
