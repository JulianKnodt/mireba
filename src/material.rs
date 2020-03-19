use crate::{
  vec::{Ray, Vec3, Vector},
  vis::Visibility,
};
use num::{Float, One, Zero};
use rand::prelude::*;
use rand_distr::{Standard, StandardNormal};

// TODO I need to remove attenuation being optional and also need to make rays optional and an
// iterator. There could be multiple outputs rays, as well as rays that pass through the
// material.

// is there some way to combine different materials components so that we can have the
// reflectance of one and emittance of another?

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
  // return attenuation, iterator over reflections and refractions
  fn scatter(&self, ray_in: &Ray<T>, vis: &Visibility<T>) -> Vec3<T>;
  /// Returns a possible reflected ray if one exists
  // TODO think about this API design as it's a little clunky without impl Iterator
  fn reflected(&self, _: &Ray<T>, _: &Visibility<T>) -> Option<Ray<T>> { None }

  /// Light emitted from this material if any
  fn emitted(&self, _: &Ray<T>, _: &Visibility<T>) -> Option<Vec3<T>> { None }
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
  fn scatter(&self, _: &Ray<T>, _: &Visibility<T>) -> Vec3<T> { self.albedo }
  fn reflected(&self, _: &Ray<T>, vis: &Visibility<T>) -> Option<Ray<T>> {
    let target = vis.pos + vis.norm.norm() + rand_in_unit_sphere();
    Some(Ray::new(vis.pos, target - vis.pos))
  }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Metallic<T> {
  albedo: Vec3<T>,
  fuzz: T,
}
impl<T: Float> Metallic<T> {
  pub fn new(albedo: Vec3<T>, fuzz: T) -> Self { Metallic { albedo, fuzz } }
}

impl<T: Float> Material<T> for Metallic<T>
where
  StandardNormal: Distribution<T>,
  Standard: Distribution<T>,
{
  fn scatter(&self, _: &Ray<T>, _: &Visibility<T>) -> Vec3<T> { self.albedo }
  fn reflected(&self, r: &Ray<T>, vis: &Visibility<T>) -> Option<Ray<T>> {
    let bounce = Ray::new(
      vis.pos,
      r.dir.norm().reflect(&vis.norm.norm()) + rand_in_unit_sphere() * self.fuzz,
    );
    Some(bounce).filter(|b| b.dir.dot(&vis.norm).is_sign_positive())
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Dielectric<T> {
  refract_idx: T,
  refract_color: Vec3<T>,
}
fn schlick<T: Float>(cosi: T, refr_idx: T) -> T {
  let r0 = ((T::one() - refr_idx) / (T::one() + refr_idx)).powi(2);
  r0 + (T::one() - r0) * (T::one() - cosi).powi(5)
}
impl<T> Dielectric<T> {
  pub fn new(t: T, refract_color: Vec3<T>) -> Self {
    Dielectric {
      refract_idx: t,
      refract_color,
    }
  }
}
impl<T: Float> Material<T> for Dielectric<T>
where
  Standard: Distribution<T>,
{
  fn scatter(&self, _: &Ray<T>, _: &Visibility<T>) -> Vec3<T> { self.refract_color }
  fn reflected(&self, r: &Ray<T>, vis: &Visibility<T>) -> Option<Ray<T>> {
    let unit_norm = vis.norm.norm();
    let v = r.dir.norm().dot(&unit_norm);
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
      .unwrap_or_else(|| r.dir.norm().reflect(&unit_norm));
    Some(Ray::new(vis.pos, out))
  }
}

/// Material with sharp reflection but fast cutoff
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GGX<D> {
  gloss: D,
  tail: D,
}

impl<D: Float> Material<D> for GGX<D> {
  fn scatter(&self, r: &Ray<D>, vis: &Visibility<D>) -> Vec3<D> {
    let d = vis.norm.dot(&r.dir).powf(self.tail);
    Vec3::from(d)
  }
  fn reflected(&self, _: &Ray<D>, vis: &Visibility<D>) -> Option<Ray<D>> {
    Some(Ray::new(vis.pos, vis.norm))
  }
}

/// Simple attenuation which uses the norm of the reflection
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Light<D> {
  color: Vec3<D>,
  intensity: D,
}

impl<D> Light<D> {
  // returns a new simple material from a given vector
  pub fn new(color: Vec3<D>, intensity: D) -> Self { Self { color, intensity } }
}
impl<D: Float> Material<D> for Light<D> {
  fn scatter(&self, _: &Ray<D>, _: &Visibility<D>) -> Vec3<D> { Vec3::zero() }
  fn emitted(&self, _: &Ray<D>, _: &Visibility<D>) -> Option<Vec3<D>> {
    Some(self.color * self.intensity)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Checkers {}
impl<D: Float> Material<D> for Checkers {
  fn scatter(&self, _: &Ray<D>, vis: &Visibility<D>) -> Vec3<D> {
    let f = vis.pos.floor();
    Vec3::one() * ((f.0.abs() + f.1.abs() + f.2.abs()) % D::from(2.0).unwrap())
  }
}

/// Wrapper around all valid instances of materials.
/// Used instead of dyn material for efficiency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mat<D = f32> {
  Dielectric(Dielectric<D>),
  Metallic(Metallic<D>),
  Lambertian(Lambertian<D>),
  GGX(GGX<D>),
  Light(Light<D>),
  Checkers(Checkers),
}

const CHECKERS: Mat = Mat::Checkers(Checkers {});
#[allow(unused)]
pub const CHECKERS_REF: &Mat = &CHECKERS;

impl<D: Float> Material<D> for Mat<D>
where
  Metallic<D>: Material<D>,
  Dielectric<D>: Material<D>,
  Lambertian<D>: Material<D>,
  Light<D>: Material<D>,
  GGX<D>: Material<D>,
{
  fn scatter(&self, r: &Ray<D>, vis: &Visibility<D>) -> Vec3<D> {
    match self {
      Mat::Dielectric(ref di) => di.scatter(r, vis),
      Mat::Metallic(ref m) => m.scatter(r, vis),
      Mat::Lambertian(ref l) => l.scatter(r, vis),
      Mat::Light(ref s) => s.scatter(r, vis),
      Mat::GGX(ref s) => s.scatter(r, vis),
      Mat::Checkers(ref c) => c.scatter(r, vis),
    }
  }
  fn reflected(&self, r: &Ray<D>, vis: &Visibility<D>) -> Option<Ray<D>> {
    match self {
      Mat::Dielectric(ref di) => di.reflected(r, vis),
      Mat::Metallic(ref m) => m.reflected(r, vis),
      Mat::Lambertian(ref l) => l.reflected(r, vis),
      Mat::Light(ref s) => s.reflected(r, vis),
      Mat::GGX(ref g) => g.reflected(r, vis),
      Mat::Checkers(ref c) => c.reflected(r, vis),
    }
  }
  fn emitted(&self, r: &Ray<D>, vis: &Visibility<D>) -> Option<Vec3<D>> {
    match self {
      Mat::Light(ref s) => s.emitted(r, vis),
      _ => None,
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
mat_from!(D, Light<D>, Mat::Light);
mat_from!(D, GGX<D>, Mat::GGX);
mat_from!(D, Checkers, Mat::Checkers);
