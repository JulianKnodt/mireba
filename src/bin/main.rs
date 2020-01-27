#![allow(unused)]

extern crate ezflags;
extern crate num;
extern crate rand;
extern crate rand_distr;

use crate::num::Zero;
use ezflags::flag::FlagSet;
use ray_weekend::{
  aabox::AABox,
  bounds::Bounds,
  indexed_triangles::from_ascii_stl,
  material::{Checkers, Dielectric, Lambertian, Mat, Metallic},
  plane::Plane,
  renderable::Renderable,
  screen::Screen,
  sphere::Sphere,
  timing::print_timed,
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
  fs.parse_args();
  let (w, h) = (w_flag.unwrap(), h_flag.unwrap());
  let n = n_flag.unwrap();

  let mut screen = Screen::new(w, h);
  let camera = Camera::aimed(
    Vec3(0., 3., 1.),
    Vec3(0., 0., -1.),
    Vec3(0., 1., 0.),
    90.,
    (w / h) as f32,
  );
  let lamb = Mat::from(Lambertian {
    albedo: Vec3(0.5, 0.5, 0.5),
  });
  let red_metal = Mat::from(Metallic::new(Vec3(1.0, 0.8, 0.5), 0.25));
  // let checkers = Mat::from(Checkers {});
  let di = Mat::from(Dielectric::new(1.5));
  let mut magnolia = from_ascii_stl("./magnolia.stl", &red_metal).unwrap();
  magnolia.shift(Vec3(0.0, -1.0, -40.0));
  magnolia.scale(0.06);
  let items: Vec<Renderable<_>> = vec![
    // Renderable::Sphere(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5, &red_metal)),
    // Renderable::Sphere(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0, &checkers)),
    // Renderable::Sphere(Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, &di)),
    // Renderable::Sphere(Sphere::new(Vec3(-2.0, 0.0, -1.0), 0.5, &lamb)),
    // Renderable::Plane(Plane::new(Vec3(0.0, 0.0, 1.0), 10.0, &red_metal)),
    Renderable::IndexedTriangles(magnolia),
  ];

  println!("Starting to render");
  (0..w).for_each(|x| {
    (0..h).for_each(|y| {
      let color = camera
        .rays(n, x as f32, y as f32, w as f32, h as f32)
        .fold(Vec3::zero(), |acc, r| {
          acc + (color(&r, &items).sqrt() * 255.9)
        })
        / (n as f32);
      screen.set(x, h - y - 1, color);
    });
  });
  screen.write_image(output_file.unwrap());
  print_timed();
}
