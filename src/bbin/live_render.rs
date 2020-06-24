#![allow(unused)]

extern crate ezflags;
extern crate image as im;
extern crate num;
extern crate piston_window;
extern crate rand;
extern crate rand_distr;
extern crate serde_json;

use crate::num::{One, Zero};
use ezflags::flag::{FlagSet, Preset};
use im::Rgba;
use piston_window::*;
use ray_weekend::{
  bounds::Bounds,
  camera::Camera,
  indexed_triangles::from_ascii_obj,
  light::{Light, PointLight},
  object::Object,
  plane::Plane,
  renderable::Renderable,
  scene::{resolve_objects, Scene},
  screen::Screen,
  sphere::Sphere,
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
  let mut n_flag = Preset(4);
  fs.add("n", "Number of rays to cast per pixel", &mut n_flag);
  let mut depth = Preset(1);
  fs.add("depth", "Depth of recursion to go until", &mut depth);
  let mut scene_src = Preset(String::from("custom.json"));
  fs.add("scene", "Scene to render", &mut scene_src);
  fs.parse_args();
  let (w, h) = (w_flag.into_inner(), h_flag.into_inner());
  let depth = depth.into_inner();
  let n = n_flag.into_inner();
  let scene_src = scene_src.into_inner();

  let mut screen = Screen::new(w, h);
  let mut camera = Camera::aimed(
    Vec3(0., 0., -5.),
    Vec3(0., 0., -3.),
    Vec3(0., 1., 0.),
    105.,
    (w / h) as f32,
  );

  println!("Loading scene from file: {}...", scene_src);
  let scene_src = std::fs::File::open(scene_src).unwrap();
  let scene: Scene<_> = serde_json::from_reader(scene_src).expect("Failed to load scene");
  let ready = scene.ready();
  // Initial render
  (0..w).for_each(|x| {
    (0..h).for_each(|y| {
      let color = camera
        .rays(n, Vec2(x as f32, y as f32))
        .fold(Vec3::zero(), |acc, r| {
          acc + ready.color_at(&r).sqrt() * 255.9
        })
        / (n as f32);
      screen.set(x, h - y - 1, color);
    });
  });
  let mut buf = screen.write_buffer();

  let mut window: PistonWindow = WindowSettings::new("Live Render", [w as u32, h as u32])
    .exit_on_esc(true)
    .build()
    .expect("Failed to build window");
  let mut texture_context = TextureContext {
    factory: window.factory.clone(),
    encoder: window.factory.create_command_buffer().into(),
  };

  let mut texture =
    Texture::from_image(&mut texture_context, &buf, &TextureSettings::new()).unwrap();
  while let Some(e) = window.next() {
    if let Some(Button::Keyboard(key)) = e.press_args() {
      match key {
        Key::Up => camera.translate(&camera.fwd()),
        Key::Down => camera.translate(&-camera.fwd()),
        _ => (),
      };
    }
    if let Some(ra) = e.render_args() {
      texture.update(&mut texture_context, &buf).unwrap();
      window.draw_2d(&e, |c, g, device| {
        texture_context.encoder.flush(device);
        clear([1.0; 4], g);
        image(&texture, c.transform, g);
      });
    }
    for x in 0..w {
      for y in 0..h {
        let color = camera
          .rays(n, Vec2(x as f32, y as f32))
          .fold(Vec3::zero(), |acc, r| {
            acc + ready.color_at(&r).sqrt() * 255.9
          })
          / (n as f32);
        let Vec3(r, g, b) = color;
        buf.put_pixel(
          x as u32,
          (h - y - 1) as u32,
          Rgba([r as u8, g as u8, b as u8, 255]),
        );
      }
    }
  }
}
