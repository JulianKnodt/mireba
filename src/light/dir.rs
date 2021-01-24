use super::Light;
use crate::{interaction::Interaction, spectrum::Spectrum};
use quick_maths::{Ray3, Vec3};

/// Represents a direction light source
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Dir {
  /// Which direction and how much to offset this light
  offset_dir: Vec3,

  /// Scale of intensity
  intensity: f32,

  /// Colour emitted by this light
  spectrum: Spectrum,
}

impl Dir {
  pub fn new(offset_dir: Vec3, intensity: f32, spectrum: Spectrum) -> Self {
    Self {
      offset_dir,
      intensity,
      spectrum,
    }
  }
}

impl Light for Dir {
  fn sample_towards(&self, it: &Interaction) -> (Ray3, Spectrum) {
    let pos = it.p - self.offset_dir;
    (
      Ray3::new(pos, self.offset_dir.norm()),
      self.spectrum * self.intensity,
    )
  }
}
