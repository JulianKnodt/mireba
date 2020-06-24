use super::{projective::ProjectiveCamera, Camera};
use crate::film::Film;
use quick_maths::{Ray, Transform4, Vec2, Vec3};

#[derive(Debug, serde::Deserialize)]
pub struct PerspectiveCamera(ProjectiveCamera);

impl PerspectiveCamera {
  pub fn new(x_fov: f32, near: f32, far: f32, aspect: f32) -> Self {
    let camera_to_raster = Transform4::scale(Vec3::new(-0.5, -0.5 * aspect, 1.0))
      * Transform4::translate(Vec3::new(-1., -1. * aspect, 0.))
      * Transform4::perspective(x_fov, near, far);
    Self(ProjectiveCamera::new(
      camera_to_raster,
      Transform4::identity(),
    ))
  }
}

impl Camera for PerspectiveCamera {
  fn sample_ray(&self, sample_pos: Vec2) -> Ray { self.0.sample_ray(sample_pos) }
  fn film(&self) -> &Film { self.0.film() }
}
