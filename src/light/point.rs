use super::Light;
use crate::{interaction::Interaction, spectrum::Spectrum};
use quick_maths::{Ray3, Vec3};

/// Represents a point light source
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Point {
  /// Position of this light
  pos: Vec3,

  /// Scale of intensity
  intensity: f32,

  /// Colour emitted by this light
  spectrum: Spectrum,
}

impl Point {
  pub fn new(pos: Vec3, intensity: f32, spectrum: Spectrum) -> Self {
    Self {
      pos,
      intensity,
      spectrum,
    }
  }
}

impl Light for Point {
  fn sample_towards(&self, it: &Interaction) -> (Ray3, Spectrum) {
    let d = it.p - self.pos;
    let dist = d.magn();
    (
      Ray3::new(self.pos, d / dist),
      self.spectrum * self.intensity / (dist * dist),
    )
  }
}

/*

/// Represents a point light source while rendering
#[derive(Debug, Clone)]
pub struct DirLight<T = f32> {
  pub intensity: T,
  pub direction: Vec3<T>,
}

#[derive(Debug, Clone)]
pub enum Light<T> {
  Point(PointLight<T>),
}

macro_rules! light_from {
  ($name: ty, $out: path) => {
    impl<T> From<$name> for Light<T> {
      fn from(src: $name) -> Self { $out(src) }
    }
  };
}

light_from!(PointLight<T>, Light::Point);

impl<T: Float> Emitter<T> for Light<T> {
  fn intensity(&self, v: &Vec3<T>) -> T {
    match self {
      Light::Point(p) => p.intensity(v),
    }
  }
  fn dir(&self, v: &Vec3<T>) -> Vec3<T> {
    match self {
      Light::Point(p) => p.dir(&v),
    }
  }
  fn shadow<V: Visible<T>, I: Iterator<Item = V>>(&self, at: &Vec3<T>, objects: I) -> T {
    match self {
      Light::Point(p) => p.shadow(at, objects),
    }
  }
}
*/
