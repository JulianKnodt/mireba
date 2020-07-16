use super::SamplingIntegrator;
use crate::{accelerator::Accelerator, scene::Scene, spectrum::Spectrum};
use quick_maths::{Ray, Vec2, Zero};

#[derive(Debug)]
pub struct Path {
  // TODO add other items here?
  depth: u32,
}

impl SamplingIntegrator for Path {
  fn sample<El, Acc: Accelerator>(
    &self,
    _position: Vec2,
    mut ray: &Ray,
    scene: &Scene<El, Acc>,
  ) -> Spectrum {
    let si = scene.intersect_ray(ray);
    let mut ior = 1.0;
    let mut result = Spectrum::zero();
    // This is the fold of pdf along the path
    let mut throughput = Spectrum::one();
    let mut si = scene.intersect_ray(&ray);
    'path_tracing: for _ in 0..self.depth {
      // TODO add to result if the intersection is at an emitter

      let (si, shape) = if Some(hit) = si {
        hit
      } else {
        break
      }

      let bsdf = shape.bsdf();
      for l in &scene.lights {
        let (light_ray, emitted_light) = l.sample_towards(&si.it);
        if let Some((l_si, _)) = scene.intersect_ray(&light_ray) {
          if si.it.t >= l_si.it.t + 0.001 {
            continue;
          }
        }
        let reflected = bsdf.eval(&si, -light_ray.dir);
        result += (reflected * emitted_light * throughput).max(0.);
      }

      let (bsdf_sample, pdf) = bsdf.sample_dir(si);
      throughput *= pdf;
      if throughput <= f32::EPSILON {
        break
      }
      eta *= bsdf_sample.ior;
      ray = Ray::new(si.it.pos, bsdf_sample.wo);
      si = scene.intersect_ray(ray);
      // TODO sample ray here for some emitter sampling
    }
    result

    // Attempt to compute direct lighting in scene
    for l in &scene.lights {
      let (ray, emitted_light) = l.sample_towards(&si.it);
      if emitted_light.is_zero() {
        continue;
      }
      let bsdf = s.bsdf();
      // add light from direct sources and ensure it's not negative
      let reflected = bsdf.eval(&si, -ray.dir);
      result += (reflected * emitted_light).max(0.);
    }
    result
  }
}
