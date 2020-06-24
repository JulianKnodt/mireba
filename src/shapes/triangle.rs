use linalg::{
  vec::{Ray, Vec2, Vec3, Vector},
  num::Float,
};
use crate::{
  bounds::{Bounded, Bounds},
  plane::Plane,
  vis::{Visibility, Visible},
};

#[derive(Debug, Clone, Copy)]
pub struct Triangle<V = Vec3<f32>>(pub Vec3<V>);

impl<T: Float> Triangle<Vec3<T>> {
  // Takes an owned triangle and converts it into a triangle of references
  // mainly used for testing but can be convenient if have a one-off triangle shape as well.
  pub fn as_ref(&self) -> Triangle<&'_ Vec3<T>> {
    let Vec3(v0, v1, v2) = &self.0;
    Triangle(Vec3(v0, v1, v2))
  }
}

impl<'a, T: Float> Triangle<&'a Vec3<T>> {
  #[inline]
  pub fn edge0(&self) -> Vec3<T> { (self.0).1 - (self.0).0 }
  #[inline]
  pub fn edge1(&self) -> Vec3<T> { (self.0).2 - (self.0).1 }
  #[inline]
  pub fn edge2(&self) -> Vec3<T> { (self.0).0 - (self.0).2 }
  /// Returns a non-unit normal to this triangle
  #[inline]
  pub fn normal(&self) -> Vec3<T> {
    let e0 = self.edge0();
    let e1 = self.edge1();
    e0.cross(&e1)
  }
  /// Intersection type 2
  pub fn intersect2(&self, r: &Ray<T>) -> Option<Visibility<T>> {
    let norm = self.normal().norm();
    let w = norm.dot((self.0).0);
    // TODO determine whether to tests both sides of the triangle or just one?
    let vis = Plane::new(norm, w)
      .hit(&r)
      .or_else(|| Plane::new(-norm, w).hit(r))?;
    if vis.param.is_sign_negative() {
      return None;
    }
    let p = vis.pos;
    let d0 = (self.0).0 - &p;
    let d1 = (self.0).1 - &p;
    let d2 = (self.0).2 - &p;
    if d0.cross(&d1).dot(&r.dir).is_sign_positive()
      || d1.cross(&d2).dot(&r.dir).is_sign_positive()
      || d2.cross(&d0).dot(&r.dir).is_sign_positive()
    {
      return None;
    }
    Some(vis)
  }
  pub fn moller_trumbore(&self, r: &Ray<T>) -> Option<Visibility<T>> {
    let &Triangle(Vec3(v0, v1, v2)) = self;
    let eps = T::from(0.00001).unwrap();
    let e1 = v1 - v0;
    let e2 = v2 - v0;
    let h = r.dir.cross(&e1);
    let a = e1.dot(&h);
    if a < eps || a > -eps {
      return None;
    }
    let f = a.recip();
    let s = r.pos - *v0;
    let u = f * s.dot(&h);
    if u < T::zero() || u > T::one() {
      return None;
    }
    let q = s.cross(&e1);
    let v = f * r.dir.dot(&q);
    if u < T::zero() || u + v > T::one() {
      return None;
    }
    let param = f + e2.dot(&q);
    if param < eps {
      return None;
    }
    let pos = r.at(param);
    Some(Visibility {
      param,
      pos,
      norm: self.normal().norm(),
    })
  }
  #[inline]
  pub fn area(&self) -> T { self.edge0().cross(&self.edge1()).magn() / (T::from(2.0).unwrap()) }
  /// Returns two barycentric coordinates for a point, but performs no checks whether it is in
  /// bounds or not
  pub fn barycentric(&self, p: &Vec3<T>) -> Vec2<T> {
    // https://gamedev.stackexchange.com/questions/23743/whats-the-most-efficient-way-to-find-barycentric-coordinates
    // a weird permutation to get the result to match the triangle verts.
    let &Triangle(Vec3(v1, v2, v0)) = self;
    let e0 = v1 - v0;
    let e1 = v2 - v0;
    let e2 = p - v0;
    let d00 = e0.dot(&e0);
    let d01 = e0.dot(&e1);
    let d11 = e1.dot(&e1);
    let d20 = e2.dot(&e0);
    let d21 = e2.dot(&e1);
    let denom = d00 * d11 - d01 * d01;
    let alpha = (d11 * d20 - d01 * d21) / denom;
    let beta = (d00 * d21 - d01 * d20) / denom;
    Vec2(alpha, beta)
  }
  /// Returns the point on this triangle where this barycentric coordinate is
  #[inline]
  pub fn point_from_barycentric(&self, b: &Vec3<T>) -> Vec3<T> {
    *(self.0).0 * b.0 + *(self.0).1 * b.1 + *(self.0).2 * b.2
  }
  pub fn contains(&self, v: &Vec3<T>) -> bool { self.barycentric(&v).is_valid_barycentric() }
}

impl<'a, T: Float> Bounded<Vec3<T>> for Triangle<&'a Vec3<T>> {
  #[inline]
  fn bounds(&self) -> Bounds<Vec3<T>> {
    let Triangle(Vec3(v0, v1, v2)) = self;
    let min = v0.min_parts(v1).min_parts(v2);
    let max = v0.max_parts(v1).max_parts(v2);
    Bounds::new([min, max])
  }
}

#[cfg(test)]
mod triangle_properties {
  use super::Triangle;
  use crate::vec::{Ray, Vec2, Vec3};
  use quickcheck::TestResult;
  quickcheck! {
    fn barycentric_identity(t: Triangle<Vec3<f32>>) -> bool {
      let Triangle(Vec3(v0, v1, v2)) = t;
      assert_eq!(Vec2(1., 0.), t.as_ref().barycentric(&v0));
      assert_eq!(Vec2(0., 1.), t.as_ref().barycentric(&v1));
      assert_eq!(Vec2(0., 0.), t.as_ref().barycentric(&v2));
      true
    }
  }
  quickcheck! {
    fn intersection(r: Ray<f32>, t: Triangle<Vec3<f32>>) -> TestResult {
      match t.as_ref().moller_trumbore(&r) {
        None => TestResult::discard(),
        Some(v) => TestResult::from_bool(t.as_ref().contains(&v.pos)),
      }
    }
  }
}
