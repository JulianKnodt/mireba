pub mod builder;
pub mod depth;
pub mod direct;

use crate::{
  accelerator::Accelerator,
  camera::{Camera, Cameras},
  scene::Scene,
  spectrum::Spectrum,
  utils::morton_decode,
};
use quick_maths::{Ray, Vec2, Vector, Zero};
use std::fmt::Debug;

pub trait Integrator: Debug {
  fn render<El, Acc: Accelerator>(&self, s: &Scene<El, Acc>);
}

pub trait SamplingIntegrator: Debug {
  fn sample<El, Acc: Accelerator>(
    &self,
    position: Vec2,
    ray: &Ray,
    camera: &Cameras,
    scene: &Scene<El, Acc>,
  ) -> Spectrum;
}

impl<S: SamplingIntegrator> Integrator for S {
  fn render<El, Acc: Accelerator>(&self, s: &Scene<El, Acc>) {
    // let aperture_sample = Vec2::of(0.5);
    let Vector([w, h]) = s.camera.film().size;
    /*
    let position_sample =
    s.camera.sample_ray(position_sample
    */
    let sample_count = 1;
    for i in 0..w * h {
      let (x, y) = morton_decode(i);
      if x >= w || y >= h {
        continue;
      }
      let uv = Vec2::new(x as f32 / w as f32, y as f32 / w as f32);
      // just do simple averaging for now?
      let spec = (0..sample_count)
        .map(|_| render_sample(self, s, uv))
        .fold(Spectrum::zero(), |acc, n| acc + n)
        / (sample_count as f32);
      s.camera.film().write(uv, spec);
    }
  }
}

fn render_sample<S: SamplingIntegrator, El, Acc: Accelerator>(
  s: &S,
  scene: &Scene<El, Acc>,
  pos: Vec2,
) -> Spectrum {
  let camera = &scene.camera;
  // TODO maybe this should include a weight?
  let ray = camera.sample_ray(pos);
  // Write the sample to the position
  s.sample(pos, &ray, camera, scene)
}

pub trait MonteCarloIntegrator: SamplingIntegrator {
  /// What is the maximal amount of bounces before the integration stops.
  /// There is no such thing as infinite bounces, just the max-value,
  /// because if we ever reach that point we should stop.
  fn max_depth(&self) -> u32;
  /// At what point does russian kick in and start terminating paths
  fn min_russian_roulette_depth(&self) -> u32;
  /*
  /// Returns whether or not this should terminate, with some indicating it should not and how
  /// much to divide throughput by.
  fn should_terminate(&self, depth: u32, throughput: Spectrum) -> Option<f32> {
    // TODO implement this with a nice default implementation with random sampling and the like.
    todo!()
    /*
    if depth < self.min_russian_roulette_depth() {
      return Some(1.0);
    } else if depth > self.max_depth() {
      return None;
    }
    let p = throughtput.max().min(0.95);
    Some(p).filter(|p| p > sampler.next())
    */
  }
  */
}
