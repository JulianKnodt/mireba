pub mod builder;
pub use builder::Builder;
pub mod plane;
pub mod sphere;
pub mod triangle;
pub mod triangle_list;

use crate::{
  bounds::{Bounded, Bounds3},
  bsdf::BSDFImpl,
  interaction::SurfaceInteraction,
};
use quick_maths::{Ray, Transform4};
use std::{fmt::Debug, ptr::NonNull};

/// Generic shape trait
pub trait Shape: Debug + Bounded {
  // fn sample_position(&self, sample: Vec2) -> Vec3;
  fn intersect_ray(&self, r: &Ray) -> Option<SurfaceInteraction>;
}

/// List of all currently allowed shapes
#[derive(Debug, PartialEq)]
pub enum Variant {
  Sphere(sphere::Sphere),
  Plane(plane::Plane),
  Triangle(triangle::Triangle),
  /*
  Plane,
  BBox,
  */
}

/// Intermediate shape representation with no bsdf
#[derive(Debug)]
pub struct Geometry {
  to_world: Transform4,
  variant: Variant,
}

#[derive(Debug, PartialEq)]
pub struct Shapes {
  variant: Variant,
  to_world: Transform4,
  /// Pointer into the list of non-null bsdfs
  bsdf: NonNull<BSDFImpl>,
}

impl Shapes {
  pub fn new(si: Geometry, bsdf: &mut BSDFImpl) -> Self {
    let bsdf = unsafe { NonNull::new_unchecked(bsdf) };
    let Geometry { to_world, variant } = si;
    Self {
      variant,
      to_world,
      bsdf,
    }
  }
  pub fn bsdf(&self) -> &BSDFImpl { unsafe { self.bsdf.as_ref() } }

  pub fn intersect_ray(&self, r: &Ray) -> Option<SurfaceInteraction> {
    // TODO apply world transforms here to ray and then apply to transformation later
    use Variant::*;
    match &self.variant {
      Sphere(s) => s.intersect_ray(r),
      Plane(p) => p.intersect_ray(r),
      Triangle(t) => t.intersect_ray(r),
    }
  }
  pub fn bounds(&self) -> Bounds3 {
    use Variant::*;
    match &self.variant {
      Sphere(s) => s.bounds(),
      Plane(p) => p.bounds(),
      Triangle(t) => t.bounds(),
    }
  }
}
