#![allow(unused)]
/// Buids and serializes scenes for rendering.
/// This is essentially a playground for the renderer so I don't have to recompile release mode
/// if I want to edit the scene.
extern crate ezflags;
extern crate num;
extern crate rand;
extern crate rand_distr;

use crate::num::{One, Zero};
use ezflags::flag::FlagSet;
use ray_weekend::{
  color::RGB,
  light::{Light, PointLight},
  material::{checkers, lambertian},
  object::Object,
  plane::Plane,
  renderable::{Shapes, Storage},
  scene::*,
  sphere::Sphere,
  vec::Vec3,
};

#[allow(unused)]
fn main() {
  let mut fs = FlagSet::new();
  let mut output_file = Some(String::from("custom.json"));
  fs.add("out", "Output file name", &mut output_file);
  fs.parse_args();

  let lights: Vec<Light<_>> = vec![
    PointLight {
      pos: Vec3(5.0, 5.0, 7.0),
      intensity: 1.0,
      attenuation: Vec3(0.0, 1.0, 0.0),
      color: Vec3(1.0, 1.0, 1.0),
    }
    .into(),
    PointLight {
      pos: Vec3(-5.0, -5.0, -7.0),
      intensity: 1.0,
      attenuation: Vec3(0.0, 1.0, 0.0),
      color: Vec3(1.0, 1.0, 1.0),
    }
    .into(),
  ];
  let materials = vec![
    lambertian(RGB::red().into_inner()),
    lambertian(RGB::green().into_inner()),
    lambertian(RGB::blue().into_inner()),
    lambertian(RGB::yellow().into_inner()),
    lambertian(RGB::purple().into_inner()),
    checkers(),
  ];
  let shapes: Vec<Shapes<_>> = vec![
    /*
    Plane::new(Vec3(-1., 0., 0.), 10.0).into(),
    Plane::new(Vec3(1., 0., 0.), 10.0).into(),
    Plane::new(Vec3(0., -1., 0.), 10.0).into(),
    Plane::new(Vec3(0., 1., 0.), 10.0).into(),
    Plane::new(Vec3(0., 0., -1.), 10.0).into(),
    Plane::new(Vec3(0., 0., 1.), -10.0).into(),
    */
    Sphere::new(Vec3(-3., 3., 3.), 1.).into(),
    Sphere::new(Vec3(5., 4., 3.), 1.).into(),
    Sphere::new(Vec3(-3., 4., 5.), 1.).into(),
    Sphere::new(Vec3(-3., 4., 5.), 1.).into(),
    Sphere::new(Vec3(3., -3., -3.), 1.).into(),
  ];
  let objects: Vec<Storage> = vec![
    Object::new(0, 0).into(), // r
    Object::new(1, 1).into(), // g
    Object::new(2, 2).into(), // b
    Object::new(3, 2).into(), // checker floor
    Object::new(4, 3).into(), // yellow
                              /*
                              Object::new(5, 4).into(), // purple
                              Object::new(6, 0).into(),
                              */
  ];
  let s = Scene {
    materials,
    shapes,
    objects,
    lights,
    settings: GlobalSettings::default().with_ambience(Vec3::one()),
  };
  let file = std::fs::File::create(output_file.unwrap()).unwrap();
  serde_json::to_writer_pretty(file, &s).unwrap();
}
