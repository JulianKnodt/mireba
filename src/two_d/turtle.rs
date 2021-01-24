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
pub struct Turtle {
  pub state: Ray<f32, 2>,
  saves: Vec<Ray<f32, 2>>,

  rules: HashMap<u8, TurtleInstruction>,
}

impl Turtle {
  pub fn at(position: Vec2, dir: Vec2) -> Self {
    Turtle {
      state: Ray::new(position, dir),
      saves: vec![],
      rules: HashMap::new(),
    }
  }
}

impl Turtle {
  pub fn curr_pos(&self) -> Vec2 { self.state.pos }
  pub fn curr_dir(&self) -> Vec2 { self.state.dir }
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
