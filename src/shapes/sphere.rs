use super::Shape;
use crate::{
  interaction::{Interaction, SurfaceInteraction},
  utils::quad_solve,
};
use quick_maths::{Ray, Vec2, Vec3};

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Sphere {
  center: Vec3,
  radius: f32,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32) -> Self {
    assert!(
      radius.is_sign_positive(),
      "Sphere cannot have negative radius"
    );
    Self { center, radius }
  }
  /// Returns the normal from this sphere for a point in space
  pub fn normal(&self, v: Vec3) -> Vec3 { (v - self.center).norm() }
  pub fn contains(&self, v: &Vec3) -> bool {
    (self.center - *v).sqr_magn() <= (self.radius * self.radius)
  }
}

impl Shape for Sphere {
  fn intersect_ray(&self, r: &Ray) -> Option<SurfaceInteraction> {
    let from_sphere = r.pos - self.center;
    let a = r.dir.sqr_magn();
    let b = 2.0 * r.dir.dot(&from_sphere);
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
        let p = r.at(t);
        SurfaceInteraction {
          it: Interaction { t, p },
          normal: self.normal(p),
          wi: r.dir.norm(),

          // just a placeholder
          uv: Vec2::new(0.0, 0.0),
        }
      })
  }
}
/*
*/

/*
impl<D: Float> Bounded<Vec3<D>> for Sphere<D> {
  fn bounds(&self) -> Bounds<Vec3<D>> {
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
*/
