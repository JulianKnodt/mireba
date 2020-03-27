use crate::{
  bounds::{Bounded, Bounds},
  util::quad_solve,
  vec::{Ray, Vec3, Vector},
  vis::{Visibility, Visible},
};
use num::Float;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sphere<T> {
  center: Vec3<T>,
  radius: T,
}

impl<T: Float> Sphere<T> {
  pub fn new(center: Vec3<T>, radius: T) -> Self {
    assert!(radius.is_sign_positive());
    Self { center, radius }
  }
  #[inline]
  fn normal(&self, v: Vec3<T>) -> Vec3<T> { v - self.center }
  #[allow(dead_code)]
  fn contains(&self, v: &Vec3<T>) -> bool {
    self.center.sqr_dist(&v) <= (self.radius * self.radius)
  }
}

impl<T: Float> Visible<T> for Sphere<T> {
  fn hit(&self, r: &Ray<T>) -> Option<Visibility<T>> {
    let from_sphere = r.pos - self.center;
    let a = r.dir.sqr_magn();
    let b = T::from(2.0).unwrap() * r.dir.dot(&from_sphere);
    let c = from_sphere.sqr_magn() - self.radius * self.radius;
    quad_solve(a, b, c)
      .and_then(
        |(t0, t1)| match (t0.is_sign_positive(), t1.is_sign_positive()) {
          (true, true) => Some(t0.min(t1)),
          (true, false) => Some(t0),
          (false, true) => Some(t1),
          (false, false) => None,
        },
      )
      .map(|t| {
        let pos = r.at(t);
        Visibility {
          param: t,
          pos,
          norm: self.normal(pos),
        }
      })
  }
}

impl<D: Float> Bounded<D> for Sphere<D> {
  fn bounds(&self) -> Bounds<D> {
    let min = self.center - self.radius;
    let max = self.center + self.radius;
    Bounds::new([min, max])
  }
}

#[cfg(test)]
mod test_sphere {
  use super::Sphere;
  use crate::{vec::Ray, vis::Visible};
  use quickcheck::TestResult;
  quickcheck! {
    // tests that a ray with a t inside the sphere actually hit it
    fn inside_sphere(r: Ray<f32>, t: f32, sphere: Sphere<f32>) -> TestResult {
      if t.is_sign_negative() { return TestResult::discard() };
      let inside = sphere.contains(&r.at(t));
      if !inside { return TestResult::discard() }
      TestResult::from_bool(sphere.hit(&r).is_some())
    }
  }
}
