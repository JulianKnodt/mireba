use crate::{film::Film, spectrum::Spectrum};
use quick_maths::{Ray, Vec2, Zero};
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Builder {
  pub pos: Vec2,
  pub dir: Vec2,

  pub rules: HashMap<u8, TurtleInstruction>,
}

impl From<Builder> for Turtle {
  fn from(b: Builder) -> Self {
    let Builder { pos, dir, rules } = b;
    Self {
      state: Ray::new(pos, dir),
      saves: vec![],
      rules,
    }
  }
}

/// Turtle which can move through arbitrary space
#[derive(Clone, Debug)]
pub struct Turtle<T = f32, V = Vec2<T>> {
  pub state: Ray<T, V>,
  saves: Vec<Ray<T, V>>,

  rules: HashMap<u8, TurtleInstruction>,
}

impl<T, V> Turtle<T, V> {
  pub fn at(position: V, dir: V) -> Self {
    Turtle {
      state: Ray::new(position, dir),
      saves: vec![],
      rules: HashMap::new(),
    }
  }
}

impl<T: Copy, V: Copy> Turtle<T, V> {
  pub fn curr_pos(&self) -> V { self.state.pos }
  pub fn curr_dir(&self) -> V { self.state.dir }
  pub fn save(&mut self) { self.saves.push(self.state); }
  pub fn load(&mut self) {
    if let Some(prev) = self.saves.pop() {
      self.state = prev;
    }
  }
}

impl Turtle {
  pub fn step(&mut self, v: f32) { self.state.step(v) }
  /// Draws a series of lines onto screen and ends up at the destination
  pub fn draw_lines(&mut self, lines: &[Vec2], s: &mut Film) {
    for &l in lines {
      self.state.dir = l;
      s.line(Spectrum::zero(), self.curr_pos(), self.state.at(1.0));
      self.step(1.0);
    }
  }
  pub fn follow_raw(&mut self, i: u8) {
    if let Some(&t) = self.rules.get(&i) {
      self.follow_instruction(t);
    }
  }
  pub fn follow_instruction(&mut self, t: TurtleInstruction) {
    use TurtleInstruction::*;
    match t {
      Move(amt) => self.step(amt),
      Rotate(amt) => self.state.dir = self.state.dir.rot(amt.to_radians()),
      Save => self.save(),
      Load => self.load(),
    }
  }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TurtleInit {
  pos: Vec2,
  rules: HashMap<u8, TurtleInstruction>,
}

/// Represents a way to control a turtle
#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum TurtleInstruction {
  Move(f32),
  Rotate(f32),
  Save,
  Load,
}
