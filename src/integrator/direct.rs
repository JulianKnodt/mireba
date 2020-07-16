use super::SamplingIntegrator;
use crate::{accelerator::Accelerator, camera::Cameras, scene::Scene, spectrum::Spectrum};
use quick_maths::{Ray, Vec2, Zero};

#[derive(Debug)]
pub struct Direct {
  // TODO add other items here?
}

impl SamplingIntegrator for Direct {
  fn sample<El, Acc: Accelerator>(
    &self,
    _position: Vec2,
    ray: &Ray,
    _camera: &Cameras,
    scene: &Scene<El, Acc>,
  ) -> Spectrum {
    let si = scene.intersect_ray(ray);
    let mut result = Spectrum::zero();
    let (si, s) = if let Some((si, s)) = si {
      (si, s)
    } else {
      return result;
    };
    let bsdf = s.bsdf();

    // Attempt to compute direct lighting in scene
    for l in &scene.lights {
      let (ray, emitted_light) = l.sample_towards(&si.it);
      if emitted_light.is_zero() {
        continue;
      }
      result += emitted_light * bsdf.ambient();
      if let Some((l_si, _)) = scene.intersect_ray(&ray) {
        if si.it.t >= l_si.it.t + 0.001 {
          continue;
        }
      }
      // add light from direct sources and ensure it's not negative
      let reflected = bsdf.eval(&si, -ray.dir);
      result += (reflected * emitted_light).max(0.);
    }
    result
  }
}
