#![allow(unused)]

extern crate ezflags;
extern crate num;
extern crate rand;
extern crate rand_distr;
extern crate serde_json;

use crate::num::Zero;
use ezflags::flag::{FlagSet, Preset};
use ray_weekend::{
  bounds::Bounds,
  camera::{Cam, OrthographicCamera, PerspectiveCamera},
  scene::Scene,
  screen::Screen,
  transform::Transform,
  vec::{Vec2, Vec3},
};

fn main() {
  let mut fs = FlagSet::new();
  let mut w_flag = Preset(800);
  fs.add("w", "Width of output image", &mut w_flag);
  let mut h_flag = Preset(600);
  fs.add("h", "Height of output image", &mut h_flag);
  let mut n_flag = Preset(15);
  fs.add("n", "Number of rays to cast per pixel", &mut n_flag);
  let mut output_file = ezflags::Preset(String::from("test.jpg"));
  fs.add("out", "Output file name", &mut output_file);
  let mut depth = Preset(6);
  fs.add("depth", "Depth of recursion to go until", &mut depth);
  let mut scene_src = Preset(String::from("custom.json"));
  fs.add("scene", "Scene to render", &mut scene_src);
  fs.parse_args();
  let (w, h) = (w_flag.into_inner(), h_flag.into_inner());
  let depth = depth.into_inner();
  let n = n_flag.into_inner();
  let scene_src = scene_src.into_inner();

  let mut screen = Screen::new(w, h);
  let bounds = Bounds::<Vec2<_>>::valid([Vec2(-10., -10.), Vec2(10., 10.)]);
  let camera = PerspectiveCamera::new(
    Transform::look_at(Vec3(0., 0., -3.5), Vec3(0., 0., 1.), Vec3(0., 1., 0.)),
    bounds,
    (w, h),
    0.3,
    1.0,
    90.,
  );

  let scene_src = std::fs::File::open(scene_src).unwrap();
  let scene: Scene<_> = serde_json::from_reader(scene_src).unwrap();
  let ready = scene.ready();
  println!("Starting to render...");
  for x in 0..w {
    for y in 0..h {
      let color = camera
        .rays(n, Vec2(x as f32, y as f32))
        .fold(Vec3::zero(), |acc, r| {
          let color = ready.color_at(&r);
          acc + color.sqrt()
        })
        / (n as f32);
      let color = color.clamp(0., 1.);
      screen.set(x, h - y - 1, color);
    }
  }
  println!("Done rendering.");
  screen.write_image(output_file.into_inner());
}
