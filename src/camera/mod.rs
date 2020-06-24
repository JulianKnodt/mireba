use crate::film::Film;
use quick_maths::{Ray, Vec2};
use std::fmt::Debug;

pub trait Camera: Debug {
  /// Sample a ray from the camera using the given uv in [0,1].
  fn sample_ray(&self, sample_pos: Vec2) -> Ray;
  /// Returns the film for this camera
  fn film(&self) -> &Film;
}

// TODO create a deserialization struct for cameras

#[derive(Debug, serde::Deserialize)]
pub enum Cameras {
  // TODO add camera instances here
  Perspective(perspective::PerspectiveCamera),
}

impl Cameras {
  pub fn film(&self) -> &Film {
    use Cameras::*;
    match self {
      Perspective(c) => c.film(),
    }
  }
  pub fn sample_ray(&self, sample_pos: Vec2) -> Ray {
    use Cameras::*;
    match self {
      Perspective(c) => c.sample_ray(sample_pos),
    }
  }
}

pub mod perspective;
pub mod projective;
