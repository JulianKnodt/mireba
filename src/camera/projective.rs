use super::Camera;
use quick_maths::{Ray3, Transform4, Vec2, Vec3};

/// Generic perspective camera
#[derive(Debug)]
pub struct Projective {
  pub raster_to_camera: Transform4,
  pub camera_to_raster: Transform4,
}

impl Projective {
  pub fn new(camera_to_raster: Transform4) -> Self {
    Self {
      raster_to_camera: camera_to_raster.inv(),
      camera_to_raster,
    }
  }
}

impl Camera for Projective {
  /// Returns a ray in local space
  fn sample_ray(&self, sample_pos: Vec2) -> Ray3 {
    let local_origin = Vec3::of(0.0);
    let local_dir = self
      .raster_to_camera
      .apply_point(&Vec3::new(sample_pos.x(), sample_pos.y(), 0.0))
      .norm();
    Ray3::new(local_origin, local_dir)
  }
}
