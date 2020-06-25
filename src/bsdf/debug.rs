use super::BSDF;
use crate::{
  interaction::SurfaceInteraction,
  spectrum::{self, Spectrum},
};
use quick_maths::Vec3;

#[derive(Debug)]
pub struct Debug;

impl BSDF for Debug {
  fn eval(&self, si: &SurfaceInteraction, _: Vec3) -> Spectrum {
    spectrum::from_rgb(si.normal.abs())
  }
}
