#![allow(unused)]

extern crate ezflags;
extern crate num;
extern crate rand;
extern crate rand_distr;
extern crate serde_json;

use crate::num::{One, Zero};
use ezflags::flag::{FlagSet, Preset};
use ray_weekend::{
  bounds::Bounds,
  camera::OrthographicCamera,
  indexed_triangles::{from_ascii_obj, IndexedTriangles},
  light::{Light, PointLight},
  plane::Plane,
  renderable::Storage,
  scene::{GlobalSettings, Scene},
  screen::Screen,
  transform::Transform,
  vec::{Vec2, Vec3},
  vis::trace_ray,
};

#[allow(unused)]
fn main() {
  let mut fs = FlagSet::new();
  let mut w_flag = Preset(800);
  fs.add("w", "Width of output image", &mut w_flag);
  let mut h_flag = Preset(600);
  fs.add("h", "Height of output image", &mut h_flag);
  let mut n_flag = Preset(10);
  fs.add("n", "Number of rays to cast per pixel", &mut n_flag);
  let mut output_file = ezflags::Preset(String::from("rendered_obj.jpg"));
  fs.add("out", "Output file name", &mut output_file);
  let mut depth = Preset(3);
  fs.add("depth", "Depth of recursion to go until", &mut depth);
  let mut scene_src = Preset(String::from("trumpet.obj"));
  fs.add("scene", "Scene to render", &mut scene_src);
  fs.parse_args();
  let (w, h) = (w_flag.into_inner(), h_flag.into_inner());
  let depth = depth.into_inner();
  let n = n_flag.into_inner();
  let scene_src = scene_src.into_inner();

  let mut screen = Screen::new(w, h);
  let bounds = Bounds::<Vec2<_>>::valid([Vec2(-500., -500.), Vec2(500., 500.)]);
  let camera = OrthographicCamera::new(
    Transform::look_at(Vec3(0., 0., -500.), Vec3(0., 0., 1.), Vec3(0., 1., 0.)),
    bounds,
    (w, h),
    0.3,
    1.0,
  );
  println!("Loading scene from file: {}...", scene_src);

  let lights: Vec<Light<_>> = vec![
    /*
      PointLight {
        pos: Vec3(5.0, 5.0, 7.0),
        intensity: 1.0,
        attenuation: Vec3(0.0, 1.0, 0.0),
        color: Vec3(1.0, 1.0, 1.0),
      }
      .into(),
    */
    PointLight {
      pos: Vec3(-5.0, -5.0, -7.0),
      intensity: 1.0,
      attenuation: Vec3(0.0, 1.0, 0.0),
      color: Vec3(1.0, 1.0, 1.0),
    }
    .into(),
  ];
  let s = Scene {
    materials: vec![],
    shapes: vec![],
    objects: vec![Storage::IndexedTriangles(scene_src.into())],
    lights,
    settings: GlobalSettings::default()
      .with_ambience(Vec3::one())
      .with_background(Vec3(0.2, 0.2, 0.2)),
  };
  let rs = s.ready();

  println!("Starting to render");
  for x in 0..w {
    for y in 0..h {
      let color = camera
        .rays(n, Vec2(x as f32, y as f32))
        .fold(Vec3::zero(), |acc, r| {
          let color = rs.color_at(&r);
          acc + color.sqrt()
        })
        / (n as f32);
      let color = color.clamp(0., 1.);
      screen.set(x, h - y - 1, color);
    }
  }
  screen.write_image(output_file.into_inner());
}
