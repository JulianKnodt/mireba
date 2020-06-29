use super::Shape;
use crate::{
  bounds::{Bounded, Bounds3},
  interaction::{Interaction, SurfaceInteraction},
};
use quick_maths::{Ray, Vec2, Vec3, Vector, Zero};

/// Represents a plane in 3d space
/// Defined by the equation (P * NormaL) + w = 0
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane {
  /// Point on the plane
  normal: Vec3,

  w: f32,

  /// Up on plane
  up: Vec3,
  /// right on plane
  right: Vec3,
  // normal is right x up
}

impl Plane {
  pub fn new(normal: &Vec3, w: f32, up: &Vec3, width: f32, height: f32) -> Self {
    let normal = normal.norm();
    // rebuild
    let right = normal.cross(&up).norm();
    let up = up.cross(&normal).norm();
    Self {
      normal,
      w,
      right: right * width / 2.,
      up: up * height / 2.,
    }
  }
  /// Returns a representative point on this plane
  pub fn repr_point(&self) -> Vec3 { -self.normal * self.w }
  pub fn on_plane(&self, v: &Vec3) -> bool { self.normal.dot(v) + self.w < 0.001 }
  fn uv(&self, Vector([x, y, _]): &Vec3) -> Vec2 {
    let Vector([rx, ry, _]) = self.right;
    let Vector([ux, uy, _]) = self.up;
    // [norm, right, up] * [0, u, v] = [x,y,z]
    let v = x / (ux - rx / ry * uy);
    let u = (y - uy * v) / rx;
    Vec2::new(u, v)
  }
}

impl Shape for Plane {
  fn intersect_ray(&self, r: &Ray) -> Option<SurfaceInteraction> {
    let d = self.normal.dot(&r.dir);
    if d.is_zero() {
      return None;
    }
    let t = -(r.pos.dot(&self.normal) + self.w) / d;
    if t.is_sign_negative() {
      return None;
    }
    let p = r.at(t);
    Some(SurfaceInteraction {
      it: Interaction { t, p },
      normal: self.normal,
      uv: self.uv(&p),
      wi: r.dir,
    })
  }
}

impl Bounded for Plane {
  fn bounds(&self) -> Bounds3 {
    let c = self.repr_point();
    let delta = self.right + self.up + (self.normal * 0.00001);
    Bounds3::valid(c - delta, c + delta)
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
