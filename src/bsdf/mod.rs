pub mod builder;
pub use builder::Builder;
pub mod debug;
pub mod diffuse;
pub mod mtl;
pub mod phong;

use crate::{interaction::SurfaceInteraction, spectrum::Spectrum};
use quick_maths::{Zero, Vec3};
use std::fmt::Debug;

/// Trait representing a BSDF
pub trait BSDF: Debug {
  // fn sample(&self, si: &SurfaceInteraction, wo: Vec3, dir_sample: Vec2) -> (Sample, Spectrum);
  /// Evaluate this bsdf at the surface interaction in the outgoing direction
  fn eval(&self, si: &SurfaceInteraction, wo: Vec3) -> Spectrum;
  // TODO add pdf in some outgoing direction
}

/// Different implementations of BSDFs
#[derive(Debug)]
pub enum BSDFImpl {
  Diffuse(diffuse::Diffuse),
  Debug(debug::Debug),
  MTL(mtl::MTL),
}

impl BSDFImpl {
  // TODO decide if wo should be a reference or not
  pub fn eval(&self, si: &SurfaceInteraction, wo: Vec3) -> Spectrum {
    use BSDFImpl::*;
    match self {
      Diffuse(d) => d.eval(si, wo),
      Debug(d) => d.eval(si, wo),
      MTL(mtl) => mtl.eval(si, wo),
    }
  }
  pub fn sample(&self) -> (Sample, Spectrum);

  /// Returns the ambient amont of lighting of this surface.
  pub fn ambient(&self) -> Spectrum { Spectrum::zero() }
}

#[derive(Debug)]
pub struct Sample {
  wo: Vec3,
  // Probability of sampling
  pdf: f32,
  // relative index of refraction
  eta: f32,
}
