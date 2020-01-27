use crate::{
  vec::{Ray, Vec3},
  vis::Visibility,
};
use num::{Float, One};
use rand::prelude::*;
use rand_distr::{Standard, StandardNormal};

fn rand_in_unit_sphere<T: Float>() -> Vec3<T>
where
  StandardNormal: Distribution<T>,
  Standard: Distribution<T>, {
  let std = StandardNormal;
  let mut rng = thread_rng();
  let v = Vec3(rng.sample(std), rng.sample(std), rng.sample(std)).norm();
  v / rng.gen().powf(T::from(1.0 / 3.0).unwrap())
}

pub trait Material<T> {
  // return attenuation, out ray
  fn scatter(&self, ray_in: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Lambertian<T> {
  pub albedo: Vec3<T>,
}

impl<T: Float> Material<T> for Lambertian<T>
where
  StandardNormal: Distribution<T>,
  Standard: Distribution<T>,
{
  fn scatter(&self, _: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)> {
    let target = vis.pos + vis.norm.norm() + rand_in_unit_sphere();
    Some((self.albedo, Ray::new(vis.pos, target - vis.pos)))
  }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Metallic<T> {
  albedo: Vec3<T>,
  fuzz: T,
}
impl<T: Float> Metallic<T> {
  pub fn new(albedo_color: Vec3<T>, fuzz: T) -> Self {
    Metallic {
      albedo: albedo_color,
      fuzz,
    }
  }
}

impl<T: Float> Material<T> for Metallic<T>
where
  StandardNormal: Distribution<T>,
  Standard: Distribution<T>,
{
  fn scatter(&self, r: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)> {
    let bounce = Ray::new(
      vis.pos,
      r.dir.norm().reflect(vis.norm.norm()) + rand_in_unit_sphere() * self.fuzz,
    );
    Some((self.albedo, bounce)).filter(|(_, b)| b.dir.dot(vis.norm).is_sign_positive())
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Dielectric<T> {
  refract_idx: T,
}
fn schlick<T: Float>(cosi: T, refr_idx: T) -> T {
  let r0 = ((T::one() - refr_idx) / (T::one() + refr_idx)).powi(2);
  r0 + (T::one() - r0) * (T::one() - cosi).powi(5)
}
impl<T> Dielectric<T> {
  pub fn new(t: T) -> Self { Dielectric { refract_idx: t } }
}
impl<T: Float> Material<T> for Dielectric<T>
where
  Standard: Distribution<T>,
{
  fn scatter(&self, r: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)> {
    let unit_norm = vis.norm.norm();
    let v = r.dir.norm().dot(unit_norm);
    let (out_norm, refr_ratio, cosi) = if v.is_sign_positive() {
      (
        -vis.norm.norm(),
        self.refract_idx,
        self.refract_idx * v / r.dir.magn(),
      )
    } else {
      (vis.norm.norm(), self.refract_idx.recip(), -v / r.dir.magn())
    };
    let out = r
      .dir
      .refract(out_norm, refr_ratio)
      .filter(|_| rand::random() < schlick(cosi, self.refract_idx))
      .unwrap_or_else(|| r.dir.norm().reflect(unit_norm));
    Some((Vec3::one(), Ray::new(vis.pos, out)))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Checkers {}
impl<D: Float> Material<D> for Checkers {
  fn scatter(&self, _: &Ray<D>, vis: &Visibility<D>) -> Option<(Vec3<D>, Ray<D>)> {
    let f = vis.pos.floor();
    let color = Vec3::one() * ((f.0.abs() + f.1.abs() + f.2.abs()) % D::from(2.0).unwrap());
    Some((color, Ray::new(vis.pos, vis.norm)))
  }
}

/// Wrapper around all valid instances of materials.
/// Used instead of dyn material for efficiency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mat<D = f32> {
  Dielectric(Dielectric<D>),
  Metallic(Metallic<D>),
  Lambertian(Lambertian<D>),
  Checkers(Checkers),
}

/*
const CHECKERS: Mat = Mat::Checkers(Checkers{});
pub const CHECKERS_REF: &'static Mat = &CHECKERS;
*/

impl<D: Float> Material<D> for Mat<D>
where
  Metallic<D>: Material<D>,
  Dielectric<D>: Material<D>,
  Lambertian<D>: Material<D>,
{
  fn scatter(&self, r: &Ray<D>, vis: &Visibility<D>) -> Option<(Vec3<D>, Ray<D>)> {
    match self {
      Mat::Dielectric(ref di) => di.scatter(r, vis),
      Mat::Metallic(ref m) => m.scatter(r, vis),
      Mat::Lambertian(ref l) => l.scatter(r, vis),
      Mat::Checkers(ref c) => c.scatter(r, vis),
    }
  }
}

macro_rules! mat_from {
  ($over: ty, $name: ty, $out: path) => {
    impl<D> From<$name> for Mat<D> {
      fn from(src: $name) -> Self { $out(src) }
    }
  };
}
mat_from!(D, Dielectric<D>, Mat::Dielectric);
mat_from!(D, Metallic<D>, Mat::Metallic);
mat_from!(D, Lambertian<D>, Mat::Lambertian);
mat_from!(D, Checkers, Mat::Checkers);
