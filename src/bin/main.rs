extern crate num;
extern crate rand;
extern crate rand_distr;
use crate::num::Zero;
use ray_weekend::{
  material::{Dielectric, Lambertian, Metallic},
  screen::Screen,
  sphere::Sphere,
  vec::Vec3,
  vis::{color, Camera},
  octree::{Octree},
  renderable::Renderable,
};

fn main() {
  let (w, h) = (800, 600);
  let n = 150;

  let mut screen = Screen::new(w, h);
  let camera = Camera::aimed(
    Vec3(-5., 2., 1.),
    Vec3(0., 0., -1.),
    Vec3(0., 1., 0.),
    90.,
    (w / h) as f32,
  );
  let lamb = Lambertian {
    albedo: Vec3(0.5, 0.5, 0.5),
  };
  let red_metal = Metallic::new(1.0, 0.8, 0.5, 0.25);
  let di = Dielectric::new(1.5);
  let items: Vec<Box<dyn Renderable<f32>>> = vec![
    // Box::new(from_ascii_stl("./magnolia.stl", &lamb).unwrap()),
    Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5, &red_metal)),
    Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0, &lamb)),
    Box::new(Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, &di)),
    Box::new(Sphere::new(Vec3(-2.0, 0.0, -1.0), 0.5, &lamb)),
  ];
  // let oct = items.into_iter().collect::<Octree<_,_>>();
  println!("Starting to render");
  (0..w).for_each(|x| {
    (0..h).for_each(|y| {
      let color = camera
        .rays(n, x as f32, y as f32, w as f32, h as f32)
        .iter()
        .fold(Vec3::zero(), |acc, r| {
          acc + (color(r, &items).sqrt() * 255.9)
        })
        / (n as f32);
      screen.set(x, h - y - 1, color);
    });
  });
  screen.write_image("test.jpg");
}
