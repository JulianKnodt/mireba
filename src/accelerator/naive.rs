use super::Accelerator;
use crate::{interaction::SurfaceInteraction, shapes::Shapes};
use quick_maths::Ray3;

#[derive(Debug)]
pub struct Naive {
  /// List of shapes that we maintain
  shapes: Vec<Shapes>,
}

impl Accelerator for Naive {
  fn build(i: impl Iterator<Item = Shapes>) -> Self {
    let shapes = i.collect::<Vec<_>>();
    Self { shapes }
  }
  fn intersect_ray(&self, r: &Ray3) -> Option<(SurfaceInteraction, &Shapes)> {
    self
      .shapes
      .iter()
      .filter_map(|s| s.intersect_ray(r).map(|si| (si, s)))
      .filter(|(si, _)| si.it.t > f32::EPSILON)
      .max_by(|a, b| a.0.it.t.partial_cmp(&b.0.it.t).unwrap())
  }
}
