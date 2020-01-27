use crate::{
  material::Mat,
  vec::{Ray, Vec3},
  vis::{Visibility, Visible},
};
use num::Float;

/// Represents a plane in 3d space
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Plane<'m, D> {
  normal: Vec3<D>,
  w: D,
  mat: &'m Mat<D>,
}

impl<'m, D: Float> Plane<'m, D> {
  pub fn new(normal: Vec3<D>, w: D, mat: &'m Mat<D>) -> Self { Self { normal, w, mat } }
}

impl<'m, D: Float> Visible<'m, D> for Plane<'m, D> {
  fn hit(&self, r: &Ray<D>) -> Option<Visibility<'m, D>> {
    let param = -(r.pos.dot(self.normal) + self.w) / self.normal.dot(r.dir);
    if param.is_sign_negative() {
      return None;
    }
    let pos = r.at(param);
    Some(Visibility {
      pos,
      param,
      norm: self.normal,
      mat: self.mat,
    })
  }
}

#[cfg(test)]
mod test_plane {
  use super::Plane;
  use crate::{vec::Ray, vis::Visible};
  use quickcheck::TestResult;
  quickcheck! {
    // Tests that any ray not parallel to the plane hits it
    fn hits_plane(r: Ray<f32>, plane: Plane<'static, f32>) -> TestResult {
      let pos_d = r.pos.dot(plane.normal);
      let dir_d = r.dir.dot(plane.normal);
      if pos_d.signum() ==  dir_d.signum() || dir_d == 0.0 { return TestResult::discard() }
      TestResult::from_bool(plane.hit(&r).is_some())
    }
  }
}
