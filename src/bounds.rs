use crate::vec::{Ray, Vec3};
use num::Float;
use serde::{Deserialize, Serialize};

#[inline]
fn overlaps_1d<D: Float>(a_min: D, a_max: D, b_min: D, b_max: D) -> bool {
  a_max > b_min && b_max > a_min
}

/// Axis Aligned bounding box
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bounds<Dim>([Vec3<Dim>; 2]);
impl<D: Float> Bounds<D> {
  /// Returns the minimum of this bounding box
  #[inline]
  pub fn min(&self) -> &Vec3<D> { &self.0[0] }
  /// Returns the maximum of this bounding box
  #[inline]
  pub fn max(&self) -> &Vec3<D> { &self.0[1] }
  /// Creates a new bound and panics if the bounds aren't properly min-maxed
  #[inline]
  pub fn new(a: [Vec3<D>; 2]) -> Self {
    assert!(a[0].0 <= a[1].0);
    assert!(a[0].1 <= a[1].1);
    assert!(a[0].2 <= a[1].2);
    Bounds(a)
  }
  #[inline]
  pub fn valid(a: [Vec3<D>; 2]) -> Self { Bounds([a[0].min_parts(&a[1]), a[0].max_parts(&a[1])]) }
  /// Returns whether this bounding box contains the other.
  /// If they have the same coordinates for one of the sides it will still return true
  #[inline]
  pub fn contains(&self, o: &Self) -> bool { self.max() >= o.max() && self.min() <= o.min() }

  /// whether or not this bounding box contains a vector
  #[inline]
  #[allow(unused)]
  pub fn contains_vec(&self, v: &Vec3<D>) -> bool {
    let &Vec3(x, y, z) = v;
    let &Vec3(hx, hy, hz) = self.max();
    let &Vec3(lx, ly, lz) = self.max();
    x >= lx && x <= hx && y >= ly && y <= hy && z >= lz && z <= hz
  }

  /// Returns whether edges of the other bounding box are fully contained in this one.
  #[inline]
  pub fn strictly_contains(&self, o: &Self) -> bool { self.max() > o.max() && self.min() < o.min() }
  #[inline]
  pub fn union(&self, o: &Self) -> Self {
    Self::new([self.min().min_parts(o.min()), self.max().max_parts(o.max())])
  }
  /// Returns the volume inside this bounding box
  #[inline]
  pub fn volume(&self) -> D {
    let &Vec3(lx, ly, lz) = self.min();
    let &Vec3(hx, hy, hz) = self.max();
    (hx - lx) * (hy - ly) * (hz - lz)
  }
  /// Returns whether this bounding box intersects this ray.
  /// Can possibly intersect backwards.
  pub fn intersects_ray(&self, r: &Ray<D>) -> bool {
    let &Vec3(lx, ly, lz) = self.min();
    let &Vec3(hx, hy, hz) = self.max();
    assert!(lx < hx);
    assert!(ly < hy);
    assert!(lz < hz);
    let &Vec3(px, py, pz) = &r.pos;
    let &Vec3(dx, dy, dz) = &r.dir;
    let (thx, tlx) = ((hx - px) / dx, (lx - px) / dx);
    let (thy, tly) = ((hy - py) / dy, (ly - py) / dy);
    let (thz, tlz) = ((hz - pz) / dz, (lz - pz) / dz);

    let t_min = (thx.min(tlx)).max(thy.min(tly)).max(thz.min(tlz));
    let t_max = (thx.max(tlx)).min(thy.max(tly)).min(thz.max(tlz));

    t_max >= t_min.max(D::zero())
  }
  /// Computes the distance from a ray to the box and also the normal to the box
  pub fn intersects_ray_params(&self, r: &Ray<D>) -> Option<(D, Vec3<D>)> {
    let &Vec3(lx, ly, lz) = self.min();
    let &Vec3(hx, hy, hz) = self.max();
    let &Vec3(px, py, pz) = &r.pos;
    let &Vec3(dx, dy, dz) = &r.dir;
    let (thx, tlx) = ((hx - px) / dx, (lx - px) / dx);
    let (thy, tly) = ((hy - py) / dy, (ly - py) / dy);
    let (thz, tlz) = ((hz - pz) / dz, (lz - pz) / dz);

    let t_min = (thx.min(tlx)).max(thy.min(tly)).max(thz.min(tlz));
    let t_max = (thx.max(tlx)).min(thy.max(tly)).min(thz.max(tlz));

    if t_max >= t_min.max(D::zero()) {
      let t = if t_min.is_sign_positive() {
        t_min
      } else {
        t_max
      };

      let l = D::one();
      let o = D::zero();
      let norm = match t {
        _ if t == thx => Vec3(l, o, o),
        _ if t == tlx => Vec3(-l, o, o),
        _ if t == thy => Vec3(o, l, o),
        _ if t == tly => Vec3(o, -l, o),
        _ if t == thz => Vec3(o, o, l),
        _ if t == tlz => Vec3(o, o, -l),
        _ => unreachable!(),
      };
      let norm = if t == t_max { -norm } else { norm };

      return Some((t, norm));
    }
    None
  }
  pub fn intersects_box(&self, o: &Self) -> bool {
    let &Vec3(lx, ly, lz) = self.min();
    let &Vec3(hx, hy, hz) = self.max();
    let &Vec3(olx, oly, olz) = o.min();
    let &Vec3(ohx, ohy, ohz) = o.max();
    overlaps_1d(lx, hx, olx, ohx) && overlaps_1d(ly, hy, oly, ohy) && overlaps_1d(lz, hz, olz, ohz)
  }
}

#[cfg(test)]
mod bounds_test {
  use super::Bounds;
  use crate::vec::Ray;
  use quickcheck::TestResult;
  quickcheck! {
    // Tests that a ray that lands inside the box will correctly intersect the box
    fn inside_box(r: Ray<f32>, t: f32, bounds: Bounds<f32>) -> TestResult {
      if t.is_sign_negative() { return TestResult::discard() }
      let inside = bounds.contains_vec(&r.at(t));
      if !inside { return TestResult::discard() }
      TestResult::from_bool(bounds.intersects_ray(&r))
    }
  }
}

pub trait Bounded<D> {
  /// returns the bounds for this object
  /// Should be relatively cheap so it can be called multiple times
  fn bounds(&self) -> Bounds<D>;
}

impl<D: Clone> Bounded<D> for Bounds<D> {
  fn bounds(&self) -> Bounds<D> { self.clone() }
}

impl<D: Clone> Bounded<D> for Vec3<D> {
  fn bounds(&self) -> Bounds<D> { Bounds([self.clone(), self.clone()]) }
}
