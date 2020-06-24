use super::BSDF;
use crate::{interaction::SurfaceInteraction, spectrum::Spectrum};
use quick_maths::Vec3;

#[derive(Debug, serde::Deserialize)]
pub struct Diffuse {
  reflectance: Spectrum,
}

impl BSDF for Diffuse {
  fn eval(&self, si: &SurfaceInteraction, wo: Vec3) -> Spectrum {
    let cos_i = si.normal.dot(&-si.wi).max(0.);
    let cos_o = si.normal.dot(&wo).max(0.);
    self.reflectance * (cos_o * cos_i) * std::f32::consts::FRAC_1_PI
  }
}
