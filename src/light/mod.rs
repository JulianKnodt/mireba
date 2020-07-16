pub mod dir;
pub mod point;

use crate::{interaction::Interaction, spectrum::Spectrum};
use quick_maths::Ray;
use std::fmt::Debug;

pub trait Light: Debug {
  /*
  /// Samples a position on the surface of this light
  fn sample_position(&self, sample: Vec2) -> ();
  */
  /// Casts a ray towards an interaction of the scene, returning a ray representing the
  /// direction and the light emitted towards it
  fn sample_towards(&self, it: &Interaction) -> (Ray, Spectrum);
}

// pub mod point;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Lights {
  Point(point::Point),
  Dir(dir::Dir),
}

impl Lights {
  pub fn sample_towards(&self, it: &Interaction) -> (Ray, Spectrum) {
    use Lights::*;
    match self {
      Point(p) => p.sample_towards(it),
      Dir(d) => d.sample_towards(it),
    }
  }
}
