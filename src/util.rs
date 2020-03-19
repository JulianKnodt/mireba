use crate::{
  map::Map,
  mat::Matrix2,
  vec::{Quat, Vec2, Vec3},
};
use num::{Float, Zero};

#[inline]
pub fn quad_solve<T>(a: T, b: T, c: T) -> Option<(T, T)>
where
  T: Float, {
  Some(b * b - a * c * T::from(4.0).unwrap())
    .filter(|discrim| discrim.is_sign_positive())
    .map(|discrim| discrim.sqrt())
    .map(|d| {
      let denom = T::from(2.0).unwrap() * a;
      ((-b + d) / denom, (-b - d) / denom)
    })
}

/// Triangulates a face by taking the first vertex as the pivot and making triangles between
/// adjacent pairs of vertices
#[inline]
pub fn triangulate<I: IntoIterator<Item = usize>>(v: I) -> impl Iterator<Item = Vec3<usize>> {
  let mut iter = v.into_iter();
  let first = iter.next().unwrap();
  let second = iter.next().unwrap();
  iter.scan(second, move |prev, n| {
    let face = Vec3(first, *prev, n);
    *prev = n;
    Some(face)
  })
}

/// Takes some vectors which additively sum to an arbitrary point and returns the rotation and
/// scaling operator that would map all the input vectors to sum to the given destination.
pub fn unitize<T: Float>(vecs: &[Vec3<T>], dest: &Vec3<T>) -> Quat<T> {
  assert!(!vecs.is_empty());
  let curr_dest: Vec3<T> = vecs.iter().fold(Vec3::zero(), |acc, &n| acc + n);
  curr_dest.inverse(dest)
}

/// Takes some vectors which additively sum to an arbitrary point and returns the rotation and
/// scaling operator that would map all the input vectors to sum to the given destination.
pub fn unitize_2d<T: Float>(vecs: &[Vec2<T>], dest: &Vec2<T>) -> Matrix2<T> {
  assert!(!vecs.is_empty());
  let curr_dest: Vec2<T> = vecs.iter().fold(Vec2::zero(), |acc, &n| acc + n);
  curr_dest.inverse(dest)
}

#[cfg(test)]
mod test_utils {
  use super::*;
  use crate::{
    map::Map,
    vec::{Vec2, Vec3, Vector},
  };
  use num::Zero;
  use quickcheck::TestResult;
  quickcheck! {
    fn test_unitize(vecs: Vec<Vec3<f32>>, dest: Vec3<f32>) -> TestResult {
      if vecs.is_empty() {
        return TestResult::discard();
      }
      let op = unitize(&vecs, &dest);
      let applied = vecs.iter().fold(Vec3::zero(), |acc: Vec3<f32>, &n| acc + n.apply(op));
      TestResult::from_bool((dest-applied).sqr_magn() < 0.0001)
    }
  }
  quickcheck! {
    fn test_unitize_2d(vecs: Vec<Vec2<f32>>, dest: Vec2<f32>) -> TestResult {
      if vecs.is_empty() {
        return TestResult::discard();
      }
      let op = unitize_2d(&vecs, &dest);
      let applied = vecs.iter().fold(Vec2::zero(), |acc: Vec2<f32>, &n| acc + n.apply(op));
      TestResult::from_bool((dest-applied).sqr_magn() < 0.0001)
    }
  }
}
