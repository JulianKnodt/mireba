use super::{projective::Projective, Camera};
use quick_maths::{Ray, Transform4, Vec2, Vec3};

#[derive(Debug)]
pub struct Orthographic(Projective);

impl Orthographic {
  pub fn new(near: f32, far: f32, aspect: f32) -> Self {
    let camera_to_raster = Transform4::scale(Vec3::new(-0.5, 0.5 * aspect, 1.0))
      * Transform4::translate(Vec3::new(-1., 1. * aspect, 0.))
      * Transform4::orthographic(near, far);
    Self(Projective::new(camera_to_raster))
  }
}

impl Camera for Orthographic {
  fn sample_ray(&self, sample_pos: Vec2) -> Ray { self.0.sample_ray(sample_pos) }
}

/*
#[test]
fn perspective_camera_test() {
  use quick_maths::Zero;
  let p = Perspective::new(30., 0.0001, 1000.0, 1.0);
  let middle = p.sample_ray(Vec2::of(0.5));
  assert!(middle.pos.is_zero());
  assert_eq!(middle.dir.x(), 0.0);
  assert_eq!(middle.dir.y(), 0.0);
  let ll = p.sample_ray(Vec2::of(0.0)).dir;
  let lr = p.sample_ray(Vec2::new(0.0, 1.0)).dir;
  let ul = p.sample_ray(Vec2::new(1.0, 0.0)).dir;
  let ur = p.sample_ray(Vec2::of(1.0)).dir;
  assert_eq!(ul.x(), ur.x());
  assert_eq!(ll.x(), lr.x());

  assert_eq!(ll.y(), ul.y());
  assert_eq!(lr.y(), ur.y());
  assert_eq!(ul.sqr_magn(), 1.0);
}
*/
