use super::SamplingIntegrator;
use crate::{
  accelerator::Accelerator,
  scene::Scene,
  spectrum::{self, Spectrum},
};
use quick_maths::{Ray, Vec2, Zero};

#[derive(Debug)]
pub struct Depth {
  scale: f32,
}

impl SamplingIntegrator for Depth {
  fn sample<El, Acc: Accelerator>(
    &self,
    _position: Vec2,
    ray: &Ray,
    scene: &Scene<El, Acc>,
  ) -> Spectrum {
    let si = scene.intersect_ray(ray);
    if let Some((si, _)) = si {
      spectrum::from_mono(si.it.t / self.scale)
    } else {
      Spectrum::zero()
    }
  }
}