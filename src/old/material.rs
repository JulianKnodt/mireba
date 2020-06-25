use linalg::vec::{Ray, Vec3};
use crate::{
  brdf::Illum,
  mtl::MTL,
  num::Float,
  vis::Visibility,
};
use num::{One, Zero};

// is there some way to combine different materials components so that we can have the
// reflectance of one and emittance of another?

/*
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
*/

pub trait Material<T: Zero + One> {
  /// How much diffuse light is retained
  fn diffuse_refl(&self) -> Vec3<T> { Vec3::zero() }
  /// How much ambient light is retained
  fn ambient_refl(&self) -> Vec3<T> { Vec3::zero() }
  /// How much specular light is retained
  fn specular_refl(&self) -> Vec3<T> { Vec3::zero() }
  /// How much transparent light is retained
  fn transparent_refl(&self) -> Vec3<T> { Vec3::zero() }

  /// How shiny is this material?
  fn shine(&self) -> T { T::one() }

  // TODO add something for brdf

  /// Returns a possible reflected ray if one exists
  fn reflected(&self, _eye: &Ray<T>, _vis: &Visibility<T>) -> Option<Ray<T>> { None }

  /// Returns a possible reflected ray if one exists
  fn refracted(&self, _eye: &Ray<T>, _vis: &Visibility<T>) -> Option<Ray<T>> { None }
}

pub fn lambertian<T: Float>(albedo: Vec3<T>) -> MTL<T> { MTL::empty().diffuse(albedo).illum(1) }
pub fn checkers<T: Float>() -> MTL<T> { MTL::empty().brdf(Illum::Checkers) }
/*
impl<T: Float> Material<T> for Lambertian<T>
where
  StandardNormal: Distribution<T>,
  Standard: Distribution<T>,
{
  fn diffuse_refl(&self) -> Vec3<T> { self.albedo }
  fn reflected(&self, _: &Ray<T>, vis: &Visibility<T>) -> Option<Ray<T>> {
    let target = vis.pos + vis.norm.norm() + rand_in_unit_sphere();
    Some(Ray::new(vis.pos, target - vis.pos))
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
  fn diffuse_refl(&self) -> Vec3<T> { self.albedo }
  fn reflected(&self, r: &Ray<T>, vis: &Visibility<T>) -> Option<Ray<T>> {
    let bounce = Ray::new(
      vis.pos,
      r.dir.norm().reflect(&vis.norm.norm()) + rand_in_unit_sphere() * self.fuzz,
    );
    Some(bounce).filter(|b| b.dir.dot(&vis.norm).is_sign_positive())
  }
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
  fn specular_refl(&self) -> Vec3<T> { self.refract_color }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkers;
impl<D: Float> Material<D> for Checkers {}

/// Wrapper around all valid instances of materials.
/// Used instead of dyn material for efficiency
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Mat<D = f32> {
  Dielectric(Dielectric<D>),
  Metallic(Metallic<D>),
  Lambertian(Lambertian<D>),
  Checkers(Checkers),
  MTL(MTL<D>),
}

const CHECKERS: Mat = Mat::Checkers(Checkers);
#[allow(unused)]
pub const CHECKERS_REF: &Mat = &CHECKERS;

impl<D: Float> Material<D> for Mat<D>
where
  Metallic<D>: Material<D>,
  Dielectric<D>: Material<D>,
  Lambertian<D>: Material<D>,
{
  fn diffuse_refl(&self) -> Vec3<D> {
    match self {
      Mat::Dielectric(ref di) => di.diffuse_refl(),
      Mat::Metallic(ref m) => m.diffuse_refl(),
      Mat::Lambertian(ref l) => l.diffuse_refl(),
      // Mat::GGX(ref s) => s.diffuse_refl(),
      Mat::Checkers(ref c) => c.diffuse_refl(),
      Mat::MTL(ref m) => m.diffuse_refl(),
    }
  }
  fn specular_refl(&self) -> Vec3<D> {
    match self {
      Mat::Dielectric(ref di) => di.specular_refl(),
      Mat::Metallic(ref m) => m.specular_refl(),
      Mat::Lambertian(ref l) => l.specular_refl(),
      // Mat::GGX(ref s) => s.specular_refl(),
      Mat::Checkers(ref c) => c.specular_refl(),
      Mat::MTL(ref m) => m.specular_refl(),
    }
  }
  fn ambient_refl(&self) -> Vec3<D> {
    match self {
      Mat::Dielectric(ref di) => di.ambient_refl(),
      Mat::Metallic(ref m) => m.ambient_refl(),
      Mat::Lambertian(ref l) => l.ambient_refl(),
      // Mat::GGX(ref s) => s.ambient_refl(),
      Mat::Checkers(ref c) => c.ambient_refl(),
      Mat::MTL(ref m) => m.ambient_refl(),
    }
  }
  fn reflected(&self, r: &Ray<D>, vis: &Visibility<D>) -> Option<Ray<D>> {
    match self {
      Mat::Dielectric(ref di) => di.reflected(r, vis),
      Mat::Metallic(ref m) => m.reflected(r, vis),
      Mat::Lambertian(ref l) => l.reflected(r, vis),
      // Mat::GGX(ref g) => g.reflected(r, vis),
      Mat::Checkers(ref c) => c.reflected(r, vis),
      Mat::MTL(ref m) => m.reflected(r, vis),
    }
  }
  fn refracted(&self, r: &Ray<D>, vis: &Visibility<D>) -> Option<Ray<D>> {
    match self {
      Mat::Dielectric(ref di) => di.refracted(r, vis),
      Mat::Metallic(ref m) => m.refracted(r, vis),
      Mat::Lambertian(ref l) => l.refracted(r, vis),
      // Mat::GGX(ref g) => g.refracted(r, vis),
      Mat::Checkers(ref c) => c.refracted(r, vis),
      Mat::MTL(ref m) => m.refracted(r, vis),
    }
  }
}

macro_rules! mat_from {
  ($name: ty, $out: path) => {
    impl<D> From<$name> for Mat<D> {
      #[inline]
      fn from(src: $name) -> Self { $out(src) }
    }
  };
}
mat_from!(Dielectric<D>, Mat::Dielectric);
mat_from!(Metallic<D>, Mat::Metallic);
mat_from!(Lambertian<D>, Mat::Lambertian);
// mat_from!(D, GGX<D>, Mat::GGX);
mat_from!(MTL<D>, Mat::MTL);
mat_from!(Checkers, Mat::Checkers);
*/
