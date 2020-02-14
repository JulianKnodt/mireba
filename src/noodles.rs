use crate::vec::Vec3;
use num::Float;

/// An abstract shape type used while generating which specifies local geometry of the shape
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
  /// Sphere with a given radius
  Sphere(f32),
  /// Cylinder with a given radius and height. The center of its bottom circular face is the
  /// position.
  Cylinder { radius: f32, height: f32 },
  /// Box with len(x), width(z), and height(y). The lower left corner of the box is the position
  /// it takes.
  Box { l: f32, w: f32, h: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShapeInstance {
  shape: Shape,
  // trans: Matrix<f32>,
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
pub struct Generator {
  pub cursor: Cursor<f32>,
  /// Output of this generator
  output: Vec<ShapeInstance>,
}

impl Generator {
  pub fn sphere(&mut self, rad: f32) {
    todo!();
  }
}

impl Generator {
  pub fn create<F>(&mut self, f: F)
  where
    F: Fn(&mut Self), {
    for _ in 0..100 {
      f(self);
    }
  }
}
