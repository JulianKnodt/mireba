#![allow(unused)]

extern crate ezflags;
extern crate num;
extern crate rand;
extern crate rand_distr;

use crate::num::{One, Zero};
use ezflags::flag::FlagSet;
use ray_weekend::{
  light::{Light, PointLight},
  material::{Dielectric, Lambertian, Mat, Metallic, Checkers},
  object::Object,
  plane::Plane,
  renderable::Renderable,
  scene::*,
  sphere::Sphere,
  vec::Vec3,
};

#[allow(unused)]
fn main() {
  let mut fs = FlagSet::new();
  let mut output_file = Some(String::from("scene.json"));
  fs.add("out", "Output file name", &mut output_file);
  fs.parse_args();

  let lights: Vec<Light<_>> = vec![PointLight {
    pos: Vec3(10.0, 10.0, -10.0),
    intensity: 1.0,
    attenuation: Vec3(0.0, 0.0, 1.0),
    color: Vec3(1.0, 1.0, 1.0),
  }
  .into()];
  let materials: Vec<Mat<_>> = vec![
    Lambertian {
      albedo: Vec3(0.7, 0.3, 0.2),
    }
    .into(),
    Checkers.into(),
    Metallic::new(Vec3(1.0, 1.0, 1.0), 0.25).into(),
    Dielectric::new(1.5, Vec3(0.5, 0.0, 0.0)).into(),
  ];
  let objects: Vec<Object<_, Renderable<_>>> = vec![
    Object::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 1.0).into(), 0),
    Object::new(Sphere::new(Vec3(-2.0, 0.0, -1.0), 0.5).into(), 1),
  ];
  let s = Scene {
    materials,
    objects,
    lights,
  };
  let file = std::fs::File::create(output_file.unwrap()).unwrap();
  serde_json::to_writer(file, &s).unwrap();
}
