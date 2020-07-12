use super::Shape;
use crate::{
  bounds::{Bounded, Bounds3},
  interaction::{Interaction, SurfaceInteraction},
};
use quick_maths::{Ray, Vec2, Vec3, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle<V = Vec3>(pub Vec3<V>);

impl Bounded for Triangle {
  fn bounds(&self) -> Bounds3 {
    let Triangle(Vector([v0, v1, v2])) = self;
    let (min, max) = v0.sift(&v1);
    let (min, _) = min.sift(&v2);
    let (_, max) = max.sift(&v2);
    Bounds3::new(min, max)
  }
}

impl Shape for Triangle {
  fn intersect_ray(&self, r: &Ray) -> Option<SurfaceInteraction> {
    let &Self(Vector([v0, v1, v2])) = self;
    let eps = 0.00001;
    let e1 = v1 - v0;
    let e2 = v2 - v0;
    let h = r.dir.cross(&e2);
    let a = e1.dot(&h);
    if a > -eps && a < eps {
      return None;
    }
    let f = a.recip();
    let s = r.pos - v0;
    let u = f * s.dot(&h);
    if u < 0. || u > 1. {
      return None;
    }
    let q = s.cross(&e1);
    let v = f * r.dir.dot(&q);
    if v < 0. || u + v > 1. {
      return None;
    }
    let t = f * e2.dot(&q);
    /*
    if t < eps {
      return None;
    }
    */
    let p = r.at(t);
    Some(SurfaceInteraction {
      it: Interaction { t, p },
      uv: Vec2::new(u, v),
      normal: self.normal().norm(),
      wi: r.dir,
    })
  }
}

impl Triangle {
  pub fn edge0(&self) -> Vec3 { (self.0)[1] - (self.0)[0] }
  pub fn edge1(&self) -> Vec3 { (self.0)[2] - (self.0)[1] }
  pub fn edge2(&self) -> Vec3 { (self.0)[0] - (self.0)[2] }
  /// Returns a non-unit normal to this triangle
  pub fn normal(&self) -> Vec3 {
    let e0 = self.edge0();
    let e1 = self.edge1();
    e0.cross(&e1)
  }
  /*
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
  */
  pub fn area(&self) -> f32 { self.edge0().cross(&self.edge1()).magn() / 2.0 }
  /// Returns two barycentric coordinates for a point, but performs no checks whether it is in
  /// bounds or not
  pub fn barycentric(&self, p: &Vec3) -> Vec2 {
    // https://gamedev.stackexchange.com/questions/23743/whats-the-most-efficient-way-to-find-barycentric-coordinates
    // a weird permutation to get the result to match the triangle verts.
    let &Self(Vector([v1, v2, v0])) = self;
    let e0 = v1 - v0;
    let e1 = v2 - v0;
    let e2 = *p - v0;
    let d00 = e0.dot(&e0);
    let d01 = e0.dot(&e1);
    let d11 = e1.dot(&e1);
    let d20 = e2.dot(&e0);
    let d21 = e2.dot(&e1);
    let denom = d00 * d11 - d01 * d01;
    let alpha = (d11 * d20 - d01 * d21) / denom;
    let beta = (d00 * d21 - d01 * d20) / denom;
    Vec2::new(alpha, beta)
  }
  /// Returns the point on this triangle where this barycentric coordinate is
  #[inline]
  pub fn point_from_barycentric(&self, b: &Vec3) -> Vec3 {
    (self.0)[0] * b.x() + (self.0)[1] * b.y() + (self.0)[2] * b.z()
  }
  // pub fn contains(&self, v: &Vec3) -> bool { self.barycentric(&v).is_valid_barycentric() }
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
