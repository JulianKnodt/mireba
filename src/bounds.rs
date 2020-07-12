use quick_maths::{Float, Ray, Vec3, Vector};
use std::fmt::Debug;

/// Returns whether two intervals overlap
fn overlaps_1d<D: Float>(a_min: D, a_max: D, b_min: D, b_max: D) -> bool {
  a_max > b_min && b_max > a_min
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OctantOrder(u8);
impl OctantOrder {
  pub const fn x_parity(self) -> bool { self.0 & 0b100 == 1 }
  pub const fn y_parity(self) -> bool { self.0 & 0b10 == 1 }
  pub const fn z_parity(self) -> bool { self.0 & 1 == 1 }
  pub const fn inner(self) -> u8 { self.0 }
  pub const fn is_valid(self) -> bool { self.0 < 8 }
  pub fn in_dir(self, dir: &Vec3<bool>) -> impl Iterator<Item = OctantOrder> + '_ {
    let dir = dir.apply_fn(|v| v as u8);
    let check_1d = |curr, dir, out| curr == out || dir == out;
    (0..8).filter_map(move |o| {
      let is_valid = check_1d(self.0 >> 2, dir.x(), o >> 2)
        && check_1d((self.0 >> 1) & 1, dir.y(), (o >> 1) & 1)
        && check_1d(self.0 & 1, dir.z(), o & 1);
      Some(OctantOrder(o)).filter(|_| is_valid)
    })
  }
}

/// Axis Aligned bounding box
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bounds<const N: usize> {
  pub min: Vector<f32, N>,
  pub max: Vector<f32, N>,
}

pub type Bounds3 = Bounds<3>;
pub type Bounds2 = Bounds<2>;

impl<const N: usize> Bounds<N> {
  pub fn new(min: Vector<f32, N>, max: Vector<f32, N>) -> Self { Self { min, max } }
  pub fn valid(a: Vector<f32, N>, b: Vector<f32, N>) -> Self {
    let (min, max) = a.sift(&b);
    Self::new(min, max)
  }
  /// Returns whether this bounding box contains the other.
  /// If they have the same coordinates for one of the sides it will still return true
  pub fn contains(&self, o: &Self) -> bool { self.max >= o.max && self.min <= o.min }

  pub fn contains_vec(&self, o: &Vector<f32, N>) -> bool {
    for i in 0..N {
      if self.min[i] > o[i] || self.max[i] < o[i] {
        return false;
      }
    }
    true
  }

  /// whether or not this bounding box contains a vector
  /// Returns whether edges of the other bounding box are fully contained in this one.
  pub fn strictly_contains(&self, o: &Self) -> bool { self.max > o.max && self.min < o.min }
  pub fn diagonal(&self) -> Vector<f32, N> { self.max - self.min }

  pub fn union(&self, o: &Self) -> Self {
    let (min, _) = self.min.sift(&o.min);
    let (_, max) = self.max.sift(&o.max);
    Self::new(min, max)
  }
  pub fn union_vec(&self, v: &Vector<f32, N>) -> Self {
    let (min, _) = self.min.sift(&v);
    let (_, max) = self.max.sift(&v);
    Self::new(min, max)
  }
  pub fn center(&self) -> Vector<f32, N> { Vector::with(|i| (self.min[i] + self.max[i]) / 2.0) }
  pub fn empty(v: Vector<f32, N>) -> Self { Bounds::new(v, v) }
}

impl Bounds3 {
  /// Returns the volume inside this bounding box
  #[inline]
  pub fn volume(&self) -> f32 {
    let Vector([lx, ly, lz]) = self.min;
    let Vector([hx, hy, hz]) = self.max;
    (hx - lx) * (hy - ly) * (hz - lz)
  }
  /// Returns whether this bounding box intersects this ray.
  /// Can possibly intersect backwards.
  pub fn intersect_ray(&self, r: &Ray) -> bool {
    let Vector([lx, ly, lz]) = self.min;
    let Vector([hx, hy, hz]) = self.max;
    let Vector([px, py, pz]) = &r.pos;
    let Vector([dx, dy, dz]) = &r.dir;
    let (thx, tlx) = ((hx - px) / dx, (lx - px) / dx);
    let (thy, tly) = ((hy - py) / dy, (ly - py) / dy);
    let (thz, tlz) = ((hz - pz) / dz, (lz - pz) / dz);

    let t_min = (thx.min(tlx)).max(thy.min(tly)).max(thz.min(tlz));
    let t_max = (thx.max(tlx)).min(thy.max(tly)).min(thz.max(tlz));

    t_max >= t_min.max(0.0)
  }
  /// Computes the distance from a ray to the box and also the normal to the box
  pub fn intersects_ray_params(&self, r: &Ray) -> Option<(f32, Vec3)> {
    let Vector([lx, ly, lz]) = self.min;
    let Vector([hx, hy, hz]) = self.max;
    let Vector([px, py, pz]) = &r.pos;
    let Vector([dx, dy, dz]) = &r.dir;
    let (thx, tlx) = ((hx - px) / dx, (lx - px) / dx);
    let (thy, tly) = ((hy - py) / dy, (ly - py) / dy);
    let (thz, tlz) = ((hz - pz) / dz, (lz - pz) / dz);

    let t_min = (thx.min(tlx)).max(thy.min(tly)).max(thz.min(tlz));
    let t_max = (thx.max(tlx)).min(thy.max(tly)).min(thz.max(tlz));

    if t_max >= t_min.max(0.0) {
      let (t, is_max) = if t_min.is_sign_positive() {
        (t_min, false)
      } else {
        (t_max, true)
      };

      let l = 1.0;
      let o = 0.0;
      let (i, j, k) = match t {
        #[allow(clippy::float_cmp)]
        _ if t == thx => (l, o, o),
        #[allow(clippy::float_cmp)]
        _ if t == tlx => (-l, o, o),
        #[allow(clippy::float_cmp)]
        _ if t == thy => (o, l, o),
        #[allow(clippy::float_cmp)]
        _ if t == tly => (o, -l, o),
        #[allow(clippy::float_cmp)]
        _ if t == thz => (o, o, l),
        #[allow(clippy::float_cmp)]
        _ if t == tlz => (o, o, -l),
        _ => unreachable!(),
      };
      let norm = Vec3::new(i, j, k);
      let norm = if is_max { -norm } else { norm };

      return Some((t, norm));
    }
    None
  }
  pub fn intersects_box(&self, o: &Self) -> bool {
    let Vector([lx, ly, lz]) = self.min;
    let Vector([hx, hy, hz]) = self.max;
    let Vector([olx, oly, olz]) = o.min;
    let Vector([ohx, ohy, ohz]) = o.max;
    overlaps_1d(lx, hx, olx, ohx) && overlaps_1d(ly, hy, oly, ohy) && overlaps_1d(lz, hz, olz, ohz)
  }
  pub fn octants(&self) -> [Self; 8] {
    let Vector([mx, my, mz]) = self.center();
    let Vector([lx, ly, lz]) = self.min;
    let Vector([ux, uy, uz]) = self.max;
    [
      Self::new(Vec3::new(lx, ly, lz), Vec3::new(mx, my, mz)),
      Self::new(Vec3::new(mx, ly, lz), Vec3::new(ux, my, mz)),
      Self::new(Vec3::new(lx, my, lz), Vec3::new(mx, uy, mz)),
      Self::new(Vec3::new(lx, ly, mz), Vec3::new(mx, my, uz)),
      //
      Self::new(Vec3::new(lx, my, mz), Vec3::new(mx, uy, uz)),
      Self::new(Vec3::new(mx, ly, mz), Vec3::new(ux, my, uz)),
      Self::new(Vec3::new(mx, my, lz), Vec3::new(ux, uy, mz)),
      Self::new(Vec3::new(mx, my, mz), Vec3::new(ux, uy, uz)),
    ]
  }
  /// Returns the octant which this point is in, assuming that the point is in the bounds
  pub fn octant_of(&self, v: &Vec3) -> OctantOrder {
    let &Vector([x, y, z]) = v;
    let Vector([lx, ly, lz]) = self.min;
    let Vector([ux, uy, uz]) = self.max;
    OctantOrder(
      (((x < (lx + ux) / 2.) as u8) << 2)
        | (((y < (ly + uy) / 2.) as u8) << 1)
        | ((z < (lz + uz) / 2.) as u8),
    )
  }
  pub const fn octant_order(&self) -> [u8; 8] {
    [
      (0 << 2) | (0 << 1) | (0 << 0),
      (1 << 2) | (0 << 1) | (0 << 0),
      (0 << 2) | (1 << 1) | (0 << 0),
      (0 << 2) | (0 << 1) | (1 << 0),
      //
      (0 << 2) | (1 << 1) | (1 << 0),
      (1 << 2) | (0 << 1) | (1 << 0),
      (1 << 2) | (1 << 1) | (0 << 0),
      (1 << 2) | (1 << 1) | (1 << 0),
    ]
  }
}

pub trait Bounded: Debug {
  // This is not a cheap call, as it expects a computation of bounds, and doesn't just
  // read from a variable. If a cheap bounds is needed, store the result from this call.
  fn bounds(&self) -> Bounds3;
}

#[cfg(test)]
mod bounds_test {
  use super::Bounds3;
  use quick_maths::Ray;
  use quickcheck::{quickcheck, TestResult};
  quickcheck! {
    // Tests that a ray that lands inside the box will correctly intersect the box
    fn inside_box(r: Ray, t: f32, bounds: Bounds3) -> TestResult {
      if t.is_sign_negative() { return TestResult::discard() }
      let inside = bounds.contains_vec(&r.at(t));
      if !inside { return TestResult::discard() }
      TestResult::from_bool(bounds.intersects_ray(&r))
    }
  }
}
