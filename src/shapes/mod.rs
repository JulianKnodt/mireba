// use quick_maths::{Vec2, Vec3};
use crate::{bsdf::BSDFImpl, interaction::SurfaceInteraction};
use quick_maths::Ray;
use std::{fmt::Debug, ptr::NonNull};

/// Generic shape trait
pub trait Shape: Debug {
  // fn sample_position(&self, sample: Vec2) -> Vec3;
  fn intersect_ray(&self, r: &Ray) -> Option<SurfaceInteraction>;
}

/// List of all currently allowed shapes
#[derive(Debug, PartialEq, serde::Deserialize)]
pub enum ShapeImpl {
  Sphere(sphere::Sphere),
  /*
  Plane,
  BBox,
  */
}

#[derive(Debug, PartialEq)]
pub struct Shapes {
  shape_impl: ShapeImpl,
  /// Pointer into the list of non-null bsdfs
  bsdf: NonNull<BSDFImpl>,
}

impl Shapes {
  pub fn new(shape_impl: ShapeImpl, bsdf: &mut BSDFImpl) -> Self {
    let bsdf = unsafe { NonNull::new_unchecked(bsdf) };
    Self { shape_impl, bsdf }
  }
  pub fn intersect_ray(&self, r: &Ray) -> Option<SurfaceInteraction> {
    use ShapeImpl::*;
    match &self.shape_impl {
      Sphere(s) => s.intersect_ray(r),
    }
  }
  pub fn bsdf(&self) -> &BSDFImpl { unsafe { self.bsdf.as_ref() } }
}

pub mod sphere;
