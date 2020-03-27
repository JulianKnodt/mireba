#![allow(unused)]

extern crate ezflags;
extern crate num;
extern crate rand;
extern crate rand_distr;
extern crate serde_json;

use crate::num::{One, Zero};
use ezflags::flag::{FlagSet, Preset};
use ray_weekend::{
  aabox::AABox,
  bounds::Bounds,
  indexed_triangles::{from_ascii_obj, from_ascii_stl},
  light::{Light, PointLight},
  material::{Dielectric, Lambertian, Mat, Metallic, CHECKERS_REF},
  object::Object,
  plane::Plane,
  renderable::Renderable,
  scene::{resolve_objects, Scene},
  screen::Screen,
  sphere::Sphere,
  vec::Vec3,
  vis::{color, Camera},
};

#[allow(unused)]
fn main() {
  let mut fs = FlagSet::new();
  let mut w_flag = Preset(800);
  fs.add("w", "Width of output image", &mut w_flag);
  let mut h_flag = Preset(600);
  fs.add("h", "Height of output image", &mut h_flag);
  let mut n_flag = Preset(35);
  fs.add("n", "Number of rays to cast per pixel", &mut n_flag);
  let mut output_file = ezflags::Preset(String::from("test.jpg"));
  fs.add("out", "Output file name", &mut output_file);
  let mut depth = Preset(6);
  fs.add("depth", "Depth of recursion to go until", &mut depth);
  let mut scene_src = Preset(String::from("scene.json"));
  fs.add("scene", "Scene to render", &mut scene_src);
  fs.parse_args();
  let (w, h) = (w_flag.into_inner(), h_flag.into_inner());
  let depth = depth.into_inner();
  let n = n_flag.into_inner();
  let scene_src = scene_src.into_inner();

  let mut screen = Screen::new(w, h);
  let camera = Camera::aimed(
    // slightly downwards angle
    Vec3(1., 2.5, 1.),
    Vec3(0.5, 0., -1.),
    Vec3(0., 1., 0.3),
    105.,
    (w / h) as f32,
  );

  println!("Loading scene from file: {}...", scene_src);
  let scene_src = std::fs::File::open(scene_src).unwrap();
  let Scene {
    materials,
    objects,
    lights,
  }: Scene<_, Renderable<f32>> = serde_json::from_reader(scene_src).unwrap();
  let objects = resolve_objects(&materials[..], objects).collect::<Vec<_>>();
  println!("Starting to render");
  (0..w).for_each(|x| {
    (0..h).for_each(|y| {
      let color = camera
        .rays(n, x as f32, y as f32, w as f32, h as f32)
        .fold(Vec3::zero(), |acc, r| {
          acc + (color(&r, objects.iter(), depth, lights.as_slice()).sqrt() * 255.9)
        })
        / (n as f32);
      screen.set(x, h - y - 1, color);
    });
  });
  screen.write_image(output_file.into_inner());
}
