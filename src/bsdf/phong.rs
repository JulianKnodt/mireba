use super::BSDF;
use crate::{interaction::SurfaceInteraction, spectrum::Spectrum};
use quick_maths::Vec3;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Phong {
  diffuse: Spectrum,
  specular: Spectrum,
  shininess: f32,
}

impl BSDF for Phong {
  fn eval(&self, si: &SurfaceInteraction, wo: Vec3) -> Spectrum {
    self.diffuse * (si.normal.dot(&wo).max(0.))
      + self.specular
        * wo
          .reflect(&si.normal)
          .dot(&-si.wi)
          .max(0.)
          .powf(self.shininess)
  }
}
