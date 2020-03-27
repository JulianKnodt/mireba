use crate::{
  vec::{Vec3, Vector},
  vis::Visibility,
};
use num::Float;
use serde::{Deserialize, Serialize};

pub trait Emitter<T> {
  /// Computes the intensity of this light at some position
  fn intensity(&self, vis: &Visibility<T>) -> T;
  /// Returns a unit vector from the position to self
  fn dir(&self, v: &Vec3<T>) -> Vec3<T>;
}

/// Represents a point light source while rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointLight<T = f32> {
  /// Position of this light
  pub pos: Vec3<T>,
  /// Intensity in range 0-1
  pub intensity: T,

  /// Coefficients for attenuation
  pub attenuation: Vec3<T>,

  pub color: Vec3<T>,
}

impl<T: Float> Emitter<T> for PointLight<T> {
  fn intensity(&self, vis: &Visibility<T>) -> T {
    let Vec3(a, b, c) = self.attenuation;
    let d = self.pos.dist(&vis.pos);
    self.intensity / (a * a * d + b * d + c)
  }
  fn dir(&self, v: &Vec3<T>) -> Vec3<T> { (self.pos - *v).norm() }
}

/// Represents a point light source while rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirLight<T = f32> {
  pub intensity: T,
  pub direction: Vec3<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
  fn intensity(&self, vis: &Visibility<T>) -> T {
    match self {
      Light::Point(p) => p.intensity(vis),
    }
  }
  fn dir(&self, v: &Vec3<T>) -> Vec3<T> {
    match self {
      Light::Point(p) => p.dir(&v),
    }
  }
}
