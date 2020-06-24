use super::Camera;
use crate::film::Film;
use quick_maths::{Ray, Transform4, Vec2, Vec3};

#[derive(Debug, serde::Deserialize)]
pub struct ProjectiveCamera {
  raster_to_camera: Transform4,
  camera_to_raster: Transform4,

  camera_to_world: Transform4,
  world_to_camera: Transform4,

  film: Film,
}

impl ProjectiveCamera {
  pub fn new(camera_to_raster: Transform4, camera_to_world: Transform4) -> Self {
    Self {
      raster_to_camera: camera_to_raster.inv(),
      camera_to_raster,

      world_to_camera: camera_to_world.inv(),
      camera_to_world,

      film: Film::empty(512, 512),
    }
  }
}

impl Camera for ProjectiveCamera {
  /// Returns a ray in world space
  fn sample_ray(&self, sample_pos: Vec2) -> Ray {
    let origin = self.camera_to_world.fwd.translation();
    let local_dir =
      self
        .raster_to_camera
        .apply_point(&Vec3::new(sample_pos.x(), sample_pos.y(), 0.0));
    let direction = self.camera_to_world.apply_vec(&local_dir);
    Ray::new(origin, direction)
  }
  fn film(&self) -> &Film { &self.film }
}
