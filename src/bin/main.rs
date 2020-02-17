#![allow(unused)]

extern crate ezflags;
extern crate num;
extern crate rand;
extern crate rand_distr;

use crate::num::{One, Zero};
use ezflags::flag::FlagSet;
use ray_weekend::{
  aabox::AABox,
  bounds::Bounds,
  indexed_triangles::{from_ascii_obj, from_ascii_stl},
  material::{Dielectric, Lambertian, Light, Mat, Metallic, CHECKERS_REF, GGX},
  plane::Plane,
  renderable::Renderable,
  screen::Screen,
  sphere::Sphere,
  vec::Vec3,
  vis::{color, Camera},
};

#[allow(unused)]
fn main() {
  let mut fs = FlagSet::new();
  let mut w_flag = Some(800);
  fs.add("w", "Width of output image", &mut w_flag);
  let mut h_flag = Some(600);
  fs.add("h", "Height of output image", &mut h_flag);
  let mut n_flag = Some(35);
  fs.add("n", "Number of rays to cast per pixel", &mut n_flag);
  let mut output_file = Some(String::from("test.jpg"));
  fs.add("out", "Output file name", &mut output_file);
  let mut depth = Some(6);
  fs.add("depth", "Depth of recursion to go until", &mut depth);
  fs.parse_args();
  let (w, h) = (w_flag.unwrap(), h_flag.unwrap());
  let depth = depth.unwrap();
  let n = n_flag.unwrap();

  let mut screen = Screen::new(w, h);
  let camera = Camera::aimed(
    // slightly downwards angle
    Vec3(1., 2.5, 1.),
    Vec3(0.5, 0., -1.),
    Vec3(0., 1., 0.3),
    105.,
    (w / h) as f32,
  );
  let lamb = Mat::from(Lambertian {
    albedo: Vec3(0.5, 0.5, 0.5),
  });
  let red_metal = Mat::from(Metallic::new(Vec3(1.0, 0.8, 0.5), 0.25));
  // let checkers = Mat::from(Checkers {});
  let di = Mat::from(Dielectric::new(1.5, Vec3(0.5, 0.0, 0.0)));
  let light = Mat::from(Light::new(Vec3::one(), 100.0));
  /*
  let mut magnolia = from_ascii_stl("./magnolia.stl", &red_metal).unwrap();
  magnolia.shift(Vec3(0.0, -1.0, -40.0));
  magnolia.scale(0.06);
  */
  let bounds = Bounds::new([Vec3(-0.5, -0.6, -0.7), Vec3(1.5, 1.6, 1.7)]);
  let mut teapot = from_ascii_obj("./teapot.obj", &light).unwrap();
  teapot.shift(Vec3(-3.0, -1.0, -40.0));
  teapot.scale(0.03);
  let items: Vec<Renderable<_>> = vec![
    Renderable::Sphere(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5, &red_metal)),
    Renderable::Sphere(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0, &lamb)),
    Renderable::Sphere(Sphere::new(Vec3(1.0, 2.0, -1.0), 0.2, &light)),
    Renderable::Sphere(Sphere::new(Vec3(-2.0, 0.0, -1.0), 0.5, &lamb)),
    // Renderable::IndexedTriangles(teapot),
  ];

  println!("Starting to render");
  (0..w).for_each(|x| {
    (0..h).for_each(|y| {
      let color = camera
        .rays(n, x as f32, y as f32, w as f32, h as f32)
        .fold(Vec3::zero(), |acc, r| {
          acc + (color(&r, &items, depth).sqrt() * 255.9)
        })
        / (n as f32);
      screen.set(x, h - y - 1, color);
    });
  });
  screen.write_image(output_file.unwrap());
}
