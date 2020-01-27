use crate::{material::Mat, renderable::Renderable, sphere::Sphere, vec::Vec3};
use num::Float;

/// A trait for outputting some set of materials
pub trait MaterialCursor<'m> {
  /// returns the next material and modifies the internal state.
  fn mat(&self) -> &'m Mat<f32>;
}

/// Current position of the noodle engine
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor<D> {
  pub pos: Vec3<D>,
  pub rot: Vec3<D>,
  pub scale: Vec3<D>,
}

impl<D: Float> Cursor<D> {
  /// Adds some translation to the current cursor
  pub fn trans(&mut self, v: Vec3<D>) { self.pos = self.pos + v; }
  /// Adds rotation to the current rotation
  pub fn rot(&mut self, v: Vec3<D>) { self.rot = self.rot + v; }
  /// Multiplies the current scale by the input scale
  pub fn scale(&mut self, v: Vec3<D>) { self.scale = self.scale * v; }
}

// TODO figure out how to do some sort of material cursor?

#[derive(Debug, PartialEq)]
pub struct Generator<'m, M: MaterialCursor<'m>> {
  pub cursor: Cursor<f32>,
  pub mat_cursor: M,
  /// Output of this generator
  output: Vec<Renderable<'m, f32>>,
}

impl<'m, M: MaterialCursor<'m>> Generator<'m, M> {
  pub fn sphere(&mut self, rad: f32) {
    let mat_ref = self.mat_cursor.mat();
    self.output.push(Renderable::Sphere(Sphere::new(
      self.cursor.pos,
      rad,
      mat_ref,
    )));
  }
}

impl<'m, M: MaterialCursor<'m>> Generator<'m, M> {
  pub fn create<F>(&mut self, f: F)
  where
    F: Fn(&mut Self), {
    for _ in 0..100 {
      f(self);
    }
  }
}
