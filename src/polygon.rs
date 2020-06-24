use linalg::{
  map::Map,
  vec::{Ray, Vec2, Vector},
};
use num::{traits::float::FloatConst, Float};

/// An arbitrary n-gon in 2d.
#[derive(Debug, PartialEq, Eq)]
pub struct Polygon<T, const N: usize> {
  /// A ray descriptor of this polygon.
  /// Encodes the radius, orientation, and location of the polygon.
  descriptor: Ray<T, Vec2<T>>,
}

impl<T: Float + FloatConst, const N: usize> Polygon<T, N> {
  pub fn new(center: Vec2<T>, radius: T) -> Self {
    assert!(N > 0);
    let mut descriptor = Ray::new(center, Vec2(T::zero(), T::one()));
    descriptor.set_length(radius);
    Self { descriptor }
  }
  /// Gives the radius of this polygon
  pub fn radius(&self) -> T { self.descriptor.dir.magn() }
  /// Returns the side length of the polygon
  pub fn side_length(&self) -> T {
    self.radius() * T::from(2.0).unwrap() * (T::PI() / T::from(N).unwrap()).sin()
  }
  /// The radians of rotation at each vertex
  fn inner_angle() -> T { T::from(2.0).unwrap() * T::PI() / T::from(N).unwrap() }
  pub fn iter(&self) -> PolyVertIter<'_, T, N> {
    let pos = self.descriptor.at(T::one());
    let dir = self
      .descriptor
      .dir
      .norm()
      .apply(T::PI() - Self::inner_angle() / T::from(2.0).unwrap());
    PolyVertIter {
      poly: &self,
      curr: 0,
      curr_orient: Ray::new(pos, dir),
      side_len: self.side_length(),
    }
  }
  // TODO create some way to generate meshes from polygons by sweeping?
}

/// An iterator over the points in a polygon in counterclockwise order
#[derive(Debug)]
pub struct PolyVertIter<'a, T, const N: usize> {
  poly: &'a Polygon<T, N>,
  curr: usize,
  // not necessary but convenient so they don't have to be recomputed
  // location and direction for where to go in the future
  curr_orient: Ray<T, Vec2<T>>,
  side_len: T,
}

impl<'a, T: Float + FloatConst, const N: usize> Iterator for PolyVertIter<'a, T, N> {
  type Item = Vec2<T>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.curr == N {
      return None;
    }
    let out = self.curr_orient.pos;
    self.curr += 1;
    self.curr_orient.step(self.side_len);
    self.curr_orient.dir = self.curr_orient.dir.apply(Polygon::<T, N>::inner_angle());
    Some(out)
  }
}

#[test]
fn test_square() {
  use num::Zero;
  let square: Polygon<f32, 4> = Polygon::new(Vec2::zero(), 1f32);
  use std::f32::consts::FRAC_PI_2;
  assert_eq!(Polygon::<f32, 4>::inner_angle(), FRAC_PI_2);
  let verts = square.iter().collect::<Vec<_>>();
  assert_eq!(verts.len(), 4);
  assert!(verts.iter().any(|&v| (v - Vec2(1.0, 0.0)).magn() < 0.0005))
}
