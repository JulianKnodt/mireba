#![allow(unused)]

extern crate ezflags;
extern crate rand;

use ezflags::flag::FlagSet;
use ray_weekend::{
  color::Color, lgram::LGrammar, map::Map, screen::Screen, turtle::Turtle, util::unitize_2d,
  vec::Vec2,
};
use std::f32::consts::PI;

fn main() {
  let mut fs = FlagSet::new();
  let mut w_flag = Some(1200.0);
  fs.add("w", "width of output image", &mut w_flag);
  let mut out_file = Some(String::from("tesselate.jpg"));
  fs.add("out", "Output file name", &mut out_file);
  let mut i = Some(12);
  fs.add("i", "Number of tesselations to perform", &mut i);
  fs.parse_args();
  let w = w_flag.unwrap();
  let h = w;
  let mut screen = Screen::new(w as usize, h as usize);
  let y_offset = Vec2(1.0, 50.0);
  let x_offset = Vec2(50.0, 5.0);
  screen.fill(Color::tone(1.0).val());
  let mut turtle: Turtle<f32, _> = Turtle::at(Vec2(-100f32, -100f32), Vec2(0.0, 1.0));
  let x_vecs = vec![Vec2(0.3, 1.5), Vec2(1.0, 0.2), Vec2(0.5, -0.3)];
  let op = unitize_2d(&x_vecs, &x_offset);
  let x_vecs = x_vecs.into_iter().map(|v| v.apply(op)).collect::<Vec<_>>();
  let y_vecs = vec![
    Vec2(0.3, 0.7),
    Vec2(-0.5, 0.3),
    Vec2(3.9, -0.1),
    Vec2(0.3, 0.5),
  ];
  let op = unitize_2d(&y_vecs, &y_offset);
  let y_vecs = y_vecs.into_iter().map(|v| v.apply(op)).collect::<Vec<_>>();

  let i = 100;

  turtle.save();
  for n in 0..i {
    turtle.save();
    for _ in 0..(i - 1) {
      turtle.draw_lines(&x_vecs, &mut screen);
    }
    turtle.load();
    turtle.state.pos = turtle.state.pos + y_offset;
  }
  turtle.load();
  for n in 0..i {
    turtle.save();
    for _ in 0..(i - 1) {
      turtle.draw_lines(&y_vecs, &mut screen);
    }
    turtle.load();
    turtle.state.pos = turtle.state.pos + x_offset;
  }
  screen.write_image(out_file.unwrap());
}
