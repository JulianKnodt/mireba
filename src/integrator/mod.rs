use crate::{accelerator::Accelerator, scene::Scene, spectrum::Spectrum, utils::morton_decode};
use quick_maths::{Ray, Vec2, Vector, Zero};
use std::fmt::Debug;

pub mod direct;

pub trait Integrator: Debug {
  fn render<El, Acc: Accelerator>(&self, s: &Scene<El, Acc>);
}

pub trait SamplingIntegrator: Debug {
  fn sample<El, Acc: Accelerator>(
    &self,
    position: Vec2,
    ray: &Ray,
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
    let sample_count = 20;
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
  // TODO maybe this should include a weight?
  let ray = scene.camera.sample_ray(pos);
  // Write the sample to the position
  s.sample(pos, &ray, scene)
}
