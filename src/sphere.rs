use crate::{
  material::Material,
  util::quad_solve,
  vec::{Ray, Vec3},
  vis::{Visible, Visibility},
  octree::{Bounds, Bounded},
};
use num::Float;

pub struct Sphere<'a, T> {
  center: Vec3<T>,
  radius: T,
  mat: &'a dyn Material<T>,
}

impl<'a, T> Sphere<'a, T>
where
  T: Float,
{
  pub fn new(center: Vec3<T>, radius: T, mat: &'a dyn Material<T>) -> Self {
    assert!(radius.is_sign_positive());
    Sphere {
      center,
      radius,
      mat,
    }
  }
  fn normal(&self, v: Vec3<T>) -> Vec3<T> { v - self.center }
}

impl<'m, T> Visible<'m, T> for Sphere<'m, T>
where
  T: num::Float,
{
  fn hit(&self, r: &Ray<T>) -> Option<Visibility<'m, T>> {
    let from_sphere = r.pos - self.center;
    let a = r.dir.sqr_magn();
    let b = T::from(2.0).unwrap() * r.dir.dot(from_sphere);
    let c = from_sphere.sqr_magn() - self.radius.powi(2);
    quad_solve(a, b, c)
      .filter(|(t0, t1)| t0.is_sign_positive() || t1.is_sign_positive())
      .map(|(t0, t1)| {
        if t0.is_sign_negative() {
          t1
        } else if t1.is_sign_negative() {
          t0
        } else {
          t0.min(t1)
        }
      })
      .map(|t| (t, r.at(t)))
      .map(|(t, pos)| Visibility {
        param: t,
        pos,
        norm: self.normal(pos),
        mat: self.mat,
      })
  }
}

impl<'a, D: Float> Bounded<D> for Sphere<'a, D> {
  fn bounds(&self) -> Bounds<D> {
    let min = self.center - self.radius;
    let max = self.center + self.radius;
    Bounds::new([min, max])
  }
}
