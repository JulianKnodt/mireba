use crate::{
  light::{Emitter, Light},
  vec::{Ray, Vec3, Vector},
  vis::Visibility,
};
use num::{Float, One, Zero};
use rand::prelude::*;
use rand_distr::{Standard, StandardNormal};
use serde::{Deserialize, Serialize};

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
  /// Returns how much reflected light should be retained
  fn albedo(&self) -> Vec3<T>;

  /// Returns the color at visibility with the given lights
  fn color(&self, src: &Ray<T>, vis: &Visibility<T>, light: &Light<T>) -> Vec3<T>;

  /// Returns a possible reflected ray if one exists
  // TODO think about this API design as it's a little clunky without impl Iterator
  fn reflected(&self, _: &Ray<T>, _: &Visibility<T>) -> Option<Ray<T>> { None }

  /// Returns a possible reflected ray if one exists
  // TODO think about this API design as it's a little clunky without impl Iterator
  fn refracted(&self, _: &Ray<T>, _: &Visibility<T>) -> Option<Ray<T>> { None }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lambertian<T> {
  // This is both the color of the surface and what will get reflected.
  pub albedo: Vec3<T>,
}

impl<T: Float> Material<T> for Lambertian<T>
where
  StandardNormal: Distribution<T>,
  Standard: Distribution<T>,
{
  fn albedo(&self) -> Vec3<T> { self.albedo }
  fn reflected(&self, _: &Ray<T>, vis: &Visibility<T>) -> Option<Ray<T>> {
    let target = vis.pos + vis.norm.norm() + rand_in_unit_sphere();
    Some(Ray::new(vis.pos, target - vis.pos))
  }
  /// Returns the color at visibility with the given lights
  fn color(&self, src: &Ray<T>, vis: &Visibility<T>, light: &Light<T>) -> Vec3<T> {
    let intensity = light.intensity(&vis);
    let to_light = light.dir(&vis.pos);
    self.albedo * vis.norm.dot(&to_light) * intensity
  }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
  /// All reflections are ok
  fn albedo(&self) -> Vec3<T> { Vec3::one() }
  fn reflected(&self, r: &Ray<T>, vis: &Visibility<T>) -> Option<Ray<T>> {
    let bounce = Ray::new(
      vis.pos,
      r.dir.norm().reflect(&vis.norm.norm()) + rand_in_unit_sphere() * self.fuzz,
    );
    Some(bounce).filter(|b| b.dir.dot(&vis.norm).is_sign_positive())
  }
  fn color(&self, _: &Ray<T>, v: &Visibility<T>, _: &Light<T>) -> Vec3<T> { Vec3::zero() }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
  fn albedo(&self) -> Vec3<T> { self.refract_color }
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
  fn color(&self, _: &Ray<T>, v: &Visibility<T>, _: &Light<T>) -> Vec3<T> { Vec3::one() }
}

/*
/// Material with sharp reflection but fast cutoff
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GGX<D> {
  gloss: D,
  tail: D,
}

impl<D: Float> Material<D> for GGX<D> {
  fn color(&self, r: &Ray<D>, vis: &Visibility<D>) -> Vec3<D> {
    let d = vis.norm.dot(&r.dir).powf(self.tail);
    Vec3::from(d)
  }
  fn reflected(&self, _: &Ray<D>, vis: &Visibility<D>) -> Option<Ray<D>> {
    Some(Ray::new(vis.pos, vis.norm))
  }
}
*/

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkers;
impl<D: Float> Material<D> for Checkers {
  fn albedo(&self) -> Vec3<D> { Vec3::one() }
  fn color(&self, _: &Ray<D>, vis: &Visibility<D>, _: &Light<D>) -> Vec3<D> {
    let Vec3(x, y, z) = vis.pos.floor().apply_fn(|v| v.abs());
    Vec3::one() * ((x + y + z) % D::from(2.0).unwrap())
  }
}

/// Wrapper around all valid instances of materials.
/// Used instead of dyn material for efficiency
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mat<D = f32> {
  Dielectric(Dielectric<D>),
  Metallic(Metallic<D>),
  Lambertian(Lambertian<D>),
  // GGX(GGX<D>),
  Checkers(Checkers),
}

const CHECKERS: Mat = Mat::Checkers(Checkers);
#[allow(unused)]
pub const CHECKERS_REF: &Mat = &CHECKERS;

impl<D: Float> Material<D> for Mat<D>
where
  Metallic<D>: Material<D>,
  Dielectric<D>: Material<D>,
  Lambertian<D>: Material<D>,
  // GGX<D>: Material<D>,
{
  fn albedo(&self) -> Vec3<D> {
    match self {
      Mat::Dielectric(ref di) => di.albedo(),
      Mat::Metallic(ref m) => m.albedo(),
      Mat::Lambertian(ref l) => l.albedo(),
      // Mat::GGX(ref s) => s.albedo(),
      Mat::Checkers(ref c) => c.albedo(),
    }
  }
  fn reflected(&self, r: &Ray<D>, vis: &Visibility<D>) -> Option<Ray<D>> {
    match self {
      Mat::Dielectric(ref di) => di.reflected(r, vis),
      Mat::Metallic(ref m) => m.reflected(r, vis),
      Mat::Lambertian(ref l) => l.reflected(r, vis),
      // Mat::GGX(ref g) => g.reflected(r, vis),
      Mat::Checkers(ref c) => c.reflected(r, vis),
    }
  }
  fn color(&self, r: &Ray<D>, vis: &Visibility<D>, light: &Light<D>) -> Vec3<D> {
    match self {
      Mat::Dielectric(ref di) => di.color(r, vis, light),
      Mat::Metallic(ref m) => m.color(r, vis, light),
      Mat::Lambertian(ref l) => l.color(r, vis, light),
      // Mat::GGX(ref g) => g.color(r, vis, light),
      Mat::Checkers(ref c) => c.color(r, vis, light),
    }
  }
}

macro_rules! mat_from {
  ($name: ty, $out: path) => {
    impl<D> From<$name> for Mat<D> {
      fn from(src: $name) -> Self { $out(src) }
    }
  };
}
mat_from!(Dielectric<D>, Mat::Dielectric);
mat_from!(Metallic<D>, Mat::Metallic);
mat_from!(Lambertian<D>, Mat::Lambertian);
// mat_from!(D, GGX<D>, Mat::GGX);
mat_from!(Checkers, Mat::Checkers);
