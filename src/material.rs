use crate::{
  vec::{Ray, Vec3},
  vis::Visibility,
};
use num::One;
use rand::prelude::*;
use rand_distr::{Standard, StandardNormal};

fn rand_in_unit_sphere<T>() -> Vec3<T>
where
  T: num::Float,
  StandardNormal: Distribution<T>,
  Standard: Distribution<T>, {
  let std = StandardNormal;
  let mut rng = thread_rng();
  let v = Vec3(rng.sample(std), rng.sample(std), rng.sample(std)).norm(); v / rng.gen().powf(T::from(1.0 / 3.0).unwrap())
}

pub trait Material<T> {
  // return attenuation, out ray
  fn scatter(&self, ray_in: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)>;
}

impl<'a, T> Material<T> for &'a dyn Material<T> {
  fn scatter(&self, ray_in: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)> {
    (*self).scatter(ray_in, vis)
  }
}

pub struct Lambertian<T> {
  pub albedo: Vec3<T>,
}
impl<T> Copy for Lambertian<T> where T: Copy {}
impl<T> Clone for Lambertian<T>
where
  T: Clone,
{
  fn clone(&self) -> Self {
    Lambertian {
      albedo: self.albedo.clone(),
    }
  }
}

impl<T> Material<T> for Lambertian<T>
where
  T: num::Float,
  StandardNormal: Distribution<T>,
  Standard: Distribution<T>,
{
  fn scatter(&self, _: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)> {
    let target = vis.pos + vis.norm.norm() + rand_in_unit_sphere();
    Some((self.albedo, Ray::new(vis.pos, target - vis.pos)))
  }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metallic<T> {
  albedo: Vec3<T>,
  fuzz: T,
}
impl<T> Metallic<T>
where
  T: num::Float,
{
  pub fn new(r: T, g: T, b: T, f: T) -> Self {
    Metallic {
      albedo: Vec3(r, g, b),
      fuzz: f,
    }
  }
}

impl<T> Material<T> for Metallic<T>
where
  T: num::Float,
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

#[derive(Debug, Copy, Clone)]
pub struct Dielectric<T> {
  refract_idx: T,
}
fn schlick<T>(cosi: T, refr_idx: T) -> T
where
  T: num::Float, {
  let r0 = ((T::one() - refr_idx) / (T::one() + refr_idx)).powi(2);
  r0 + (T::one() - r0) * (T::one() - cosi).powi(5)
}
impl<T> Dielectric<T> {
  pub fn new(t: T) -> Self { Dielectric { refract_idx: t } }
}
impl<T> Material<T> for Dielectric<T>
where
  T: num::Float + Sync + Send,
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
