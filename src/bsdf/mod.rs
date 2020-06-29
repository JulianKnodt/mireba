pub mod builder;
pub use builder::Builder;
pub mod debug;
pub mod diffuse;
pub mod phong;

use crate::{interaction::SurfaceInteraction, spectrum::Spectrum};
use quick_maths::Vec3;
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
}

impl BSDFImpl {
  pub fn eval(&self, si: &SurfaceInteraction, wo: Vec3) -> Spectrum {
    use BSDFImpl::*;
    match self {
      Diffuse(d) => d.eval(si, wo),
      Debug(d) => d.eval(si, wo),
    }
  }
}

/*
#[derive(Debug)]
pub struct Sample {
  out: Vec3,
  // Probability of sampling
  pdf: f32,
  // relative index of refraction
  // eta: f32,
}
*/
