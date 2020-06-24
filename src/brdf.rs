use linalg::vec::{Ray, Vec3, Vector};
use linalg::num::Float;
use crate::{
  light::Emitter,
  material::Material,
  mtl::MTL,
  scene::ReadyScene,
  vis::Visibility,
};
use num::{Zero, One};
/// Hopefully define a bunch of brdf functions here

#[non_exhaustive]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Illum {
  ColorNoAmbient,
  ColorAmbient,
  Highlight,
  ReflectionTrace,
  Transparency,
  FresnelTrace,
  RefractionFresnelTrace,

  // Unique illumination model
  Checkers,
}

impl Illum {
  pub fn new(i: u8) -> Self {
    use Illum::*;
    match i {
      0 => ColorNoAmbient,
      1 => ColorAmbient,
      2 => Highlight,
      3 => ReflectionTrace,
      4 => Transparency,
      5 => FresnelTrace,
      6 => todo!(),
      7 => RefractionFresnelTrace,
      _ => todo!(),
    }
  }
  pub fn brdf<T: Float>(self) -> BRDF<T> {
    use Illum::*;
    match self {
      ColorNoAmbient => constant,
      ColorAmbient => lambertian,
      Highlight => phong,
      ReflectionTrace => todo!(),
      Transparency => todo!(),
      FresnelTrace => todo!(),
      RefractionFresnelTrace => todo!(),

      // Unique illumination model
      Checkers => checkers,
    }
  }
}

pub type BRDF<T> = fn(&Ray<T>, &MTL<T>, &Visibility<T>, &RS<T>) -> Vec3<T>;

type RS<'f, 'm, 's, T> = ReadyScene<'f, 'm, 's, T>;

pub fn debug<T: Float>(_: &Ray<T>, _: &MTL<T>, vis: &Visibility<T>, _: &RS<T>) -> Vec3<T> {
  vis.norm //.apply_fn(|v| v.abs())
}

pub fn checkers<T: Float>(_: &Ray<T>, mtl: &MTL<T>, vis: &Visibility<T>, _: &RS<T>) -> Vec3<T> {
  let mut color = mtl.diffuse_refl();
  if color.is_zero() {
    color = Vec3::one();
  }
  let Vec3(x, y, z) = vis.pos.apply_fn(|f| f.fract().abs() - T::from(0.5).unwrap());
  let m = if (x * y * z).is_sign_positive() { T::one() } else { T::zero() };
  color * m
}

pub fn constant<T: Float>(_: &Ray<T>, mat: &MTL<T>, _: &Visibility<T>, _: &RS<T>) -> Vec3<T> {
  mat.diffuse_refl()
}

pub fn lambertian<T: Float>(_: &Ray<T>, mat: &MTL<T>, vis: &Visibility<T>, rs: &RS<T>) -> Vec3<T> {
  let mut out = mat.ambient_refl() * rs.ambient_illumination();
  let diffuse_refl = mat.diffuse_refl();
  if !diffuse_refl.is_zero() {
    for light in rs.lights {
      let color = light.color_at(&vis.pos);
      let to_light = light.dir(&vis.pos);
      out = out + diffuse_refl * vis.norm.dot(&to_light) * color;
    }
  }
  out
}

/// Computes the phong brdf of a given material with a visibility and light
pub fn phong<T: Float>(eye: &Ray<T>, mat: &MTL<T>, vis: &Visibility<T>, rs: &RS<T>) -> Vec3<T> {
  let mut illum = mat.ambient_refl() * rs.settings.ambient_illumination;

  let to_eye = (eye.pos - vis.pos).norm();
  let diffuse_refl = mat.diffuse_refl();
  let specular_refl = mat.specular_refl();
  for light in rs.lights {
    let to_light = light.dir(&vis.pos).norm();
    let color = light.color_at(&vis.pos);
    let diffuse_intensity = vis.norm.dot(&to_light).max(T::zero());
    illum = illum + diffuse_refl * diffuse_intensity * color;
    let refl = to_light.reflect(&vis.norm);
    let specular_cont = specular_refl * refl.dot(&-to_eye).max(T::zero()).powf(mat.shine()) * color;
    illum = illum + specular_cont.clamp(T::zero(), T::one());
  }

  illum
}
