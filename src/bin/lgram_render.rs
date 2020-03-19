#![allow(unused)]

extern crate ezflags;
extern crate rand;

use ezflags::flag::FlagSet;
use ray_weekend::{color::Color, lgram::LGrammar, screen::Screen, turtle::Turtle, vec::Vec2};
use std::f32::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum States {
  X,
  F,
  P,
  M,
  // save and load
  S,
  L,
}
fn rules(v: States) -> Vec<States> {
  use States::*;
  match v {
    X => vec![
      F, P, S, M, S, X, L, M, L, F, M, F, S, F, S, M, X, L, X, L, F, P, X,
    ],
    F => vec![F, F],
    P | M | S | L => vec![v],
  }
}

fn main() {
  let mut fs = FlagSet::new();
  let mut w_flag = Some(1200.0);
  fs.add("w", "width of output image", &mut w_flag);
  let mut out_file = Some(String::from("turtle.jpg"));
  fs.add("out", "Output file name", &mut out_file);
  fs.parse_args();
  let w = w_flag.unwrap();
  let h = w;
  let mut screen = Screen::new(w as usize, h as usize);
  screen.fill(Color::rgb(0.3, 0.5, 0.7).val());
  let mut turtle: Turtle<f32, _> = Turtle::at(Vec2(w / 2f32, h / 2.0), Vec2(0., -1.));
  use States::*;
  let instrs = LGrammar::from((X, rules)).nth(6).finalize();
  for v in instrs {
    let len = 5.0;
    let color = Color::rgb(0.0, 0.8, 0.2);
    match v {
      F => {
        screen.line(turtle.curr_pos(), turtle.state.at(len), color);
        turtle.step(len);
      },
      P => {
        turtle.map(-PI * (45.0 / 180.0));
      },
      M => turtle.map(-PI * (45.0 / 180.0)),
      X => screen.circle(turtle.curr_pos(), 1.0, Color::tone(0.0)),
      S => turtle.save(),
      L => turtle.load(),
    }
  }
  screen.write_image(out_file.unwrap());
}
