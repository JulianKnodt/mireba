use crate::{
  bounds::Bounds,
  camera::{Cam, OrthographicCamera, PerspectiveCamera},
  transform::Transform,
  vec::{Vec2, Vec3},
};

#[test]
fn example_ortho() {
  let cam_to_world = Transform::look_at(Vec3(0f32, 0., 0.), Vec3(0f32, 0., 1.), Vec3(0f32, 1., 0.));
  let bounds = Bounds::<Vec2<_>>::valid([Vec2(-200., -200.), Vec2(200., 200.)]);
  let cam = OrthographicCamera::new(cam_to_world, bounds, (800, 800), 0.3, 0.9);
  let _ = cam.ray_to(Vec2(1., 1.));
}

#[test]
fn example_persp() {
  let cam_to_world = Transform::look_at(Vec3(0f32, 0., 0.), Vec3(0f32, 0., 1.), Vec3(0f32, 1., 0.));
  let bounds = Bounds::<Vec2<_>>::valid([Vec2(100., 30.), Vec2(300., 400.)]);
  let cam = PerspectiveCamera::new(cam_to_world, bounds, (800, 300), 0.3, 0.9, 90.0);
  let _ = cam.ray_to(Vec2(800., 800.));
}
