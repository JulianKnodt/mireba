use crate::{
  color::RGB,
  map::Map,
  screen::Screen,
  vec::{Ray, Vec2},
};
use num::Float;
use std::ops::{Add, Mul};

/// Turtle which can move through arbitrary space
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Turtle<D = f32, V = Vec2<D>> {
  pub state: Ray<D, V>,
  saves: Vec<Ray<D, V>>,
}

impl<D, V> Turtle<D, V> {
  pub fn at(position: V, dir: V) -> Self {
    Turtle {
      state: Ray::new(position, dir),
      saves: vec![],
    }
  }
}

impl<D: Copy, V: Copy> Turtle<D, V> {
  pub fn curr_pos(&self) -> V { self.state.pos }
  pub fn curr_dir(&self) -> V { self.state.dir }
  pub fn save(&mut self) { self.saves.push(self.state); }
  pub fn load(&mut self) {
    if let Some(prev) = self.saves.pop() {
      self.state = prev;
    }
  }
}

impl<D: Float, V> Turtle<D, V>
where
  V: Add<Output = V> + Mul<D, Output = V> + Copy,
{
  pub fn step(&mut self, v: D) { self.state.step(v) }
}

impl Turtle<f32, Vec2<f32>> {
  /// Draws a series of lines onto screen and ends up at the destination
  pub fn draw_lines(&mut self, lines: &[Vec2<f32>], s: &mut Screen) {
    for &l in lines {
      self.state.dir = l;
      s.line(self.curr_pos(), self.state.at(1.0), RGB::tone(0.0));
      self.step(1.0);
    }
  }
}

impl<D, V> Turtle<D, V> {
  pub fn map<F>(&mut self, f: F)
  where
    V: Map<F, Operator = F>, {
    self.state.dir = self.state.dir.apply(f);
  }
}
