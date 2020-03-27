use crate::{
  mat::{Matrix, Matrix4},
  vec::{Vec2, Vec3, Vec4, Vector},
};
use num::Float;
use std::{convert::TryInto, marker::PhantomData};

/// A cubic spline
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct CubicSpline<'a, D = f32, V = Vec2<D>> {
  /// Control points for use in interpolation.
  /// Borrowed because we might expect these points to often be shared
  ctrls: Vec4<&'a V>,
  _phantom: PhantomData<D>,
}

fn cubic_comp<T: Float>(u: T) -> Vec4<T> {
  let u_2 = u * u;
  Vec4([u_2 * u, u_2, u, T::one()])
}

impl<'a, D, V> CubicSpline<'a, D, V> {
  pub fn new(ctrls: &'a [V; 4]) -> Self {
    let [v0, v1, v2, v3] = ctrls;
    let ctrls = Vec4([v0, v1, v2, v3]);
    Self {
      ctrls,
      _phantom: PhantomData,
    }
  }
}

/// Creates a cubic spline of n points across ctrls with the spline variant provided.
pub fn cubic_spline<T: Float>(
  ctrls: &[Vec2<T>],
  n: usize,
  spline: Spline,
) -> impl Iterator<Item = Vec2<T>> + '_ {
  let num_win = ctrls.len().saturating_sub(3);
  assert_ne!(num_win, 0, "Did not pass enough points to cubic spline");
  let n_per = n / num_win;
  let w = spline.weights();
  ctrls
    .windows(4)
    .map(|win| CubicSpline::new(win.try_into().unwrap()))
    .flat_map(move |cs| cs.sample(n_per, w))
}

impl<'a, D: Float> CubicSpline<'a, D, Vec2<D>> {
  pub fn components(&self) -> [Vec4<D>; 2] {
    let Vec4([&Vec2(x0, y0), &Vec2(x1, y1), &Vec2(x2, y2), &Vec2(x3, y3)]) = self.ctrls;
    [Vec4([x0, x1, x2, x3]), Vec4([y0, y1, y2, y3])]
  }
  pub fn at(&self, u: D, weights: &Matrix4<D>) -> Vec2<D> {
    let us = cubic_comp(u.max(D::zero()).min(D::one()));
    let [xs, ys] = self.components();
    let x = weights.vecmul(&xs).dot(&us);
    let y = weights.vecmul(&ys).dot(&us);
    Vec2(x, y)
  }
  /// Computes n samples along this cubic spline
  pub fn sample(self, n: usize, weights: Matrix4<D>) -> impl Iterator<Item = Vec2<D>> + 'a {
    let step = D::from(n - 1).unwrap().recip();
    (0..n)
      .map(move |i| step * D::from(i).unwrap())
      .map(move |u| self.at(u, &weights))
  }
}

/// A cubic spline
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct CubicSurface<'a, D = f32, V = Vec3<D>> {
  /// Control points for use in interpolation
  ctrls: Matrix4<&'a V>,
  _phantom: PhantomData<D>,
}

impl<'a, D: Float> CubicSurface<'a, D, Vec3<D>> {
  pub fn components(&self) -> [Matrix4<D>; 3] {
    // Lol this is incredibly lazy and redundant but works
    [
      self.ctrls.apply_fn(|v| v.0),
      self.ctrls.apply_fn(|v| v.1),
      self.ctrls.apply_fn(|v| v.2),
    ]
  }
  pub fn at<M>(&self, uv: (D, D), weights: &M) -> Vec3<D>
  where
    M: Matrix<Field = D, Vector = Vec4<D>>, {
    let (u, v) = uv;
    let us = cubic_comp(u.max(D::zero()).min(D::one()));
    let vs = cubic_comp(v.max(D::zero()).min(D::one()));
    let [xs, ys, zs] = self.components();
    let partial = weights.t().vecmul(&vs);
    let x = weights.vecmul(&xs.vecmul(&partial)).dot(&us);
    let y = weights.vecmul(&ys.vecmul(&partial)).dot(&us);
    let z = weights.vecmul(&zs.vecmul(&partial)).dot(&us);
    Vec3(x, y, z)
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Spline {
  Bezier,
  B,
}

impl Spline {
  fn weights<T: Float>(&self) -> Matrix4<T> {
    use Spline::*;
    match self {
      Bezier => cubic_bezier_weights(),
      B => cubic_b_weights(),
    }
  }
}

pub fn cubic_bezier_weights<T: Float>() -> Matrix4<T> {
  let o = T::zero();
  let l = T::one();
  let t = T::from(3.0).unwrap();
  let six = T::from(6.0).unwrap();
  Matrix4([
    Vec4([-l, t, -t, l]),
    Vec4([t, -six, t, o]),
    Vec4([-t, t, o, o]),
    Vec4([l, o, o, o]),
  ])
}

pub fn cubic_b_weights<T: Float>() -> Matrix4<T> {
  let o = T::zero();
  let l = T::one();
  let t = T::from(3.0).unwrap();
  let six = T::from(6.0).unwrap();
  let four = T::from(4.0).unwrap();
  Matrix4([
    Vec4([-l, t, -t, l]),
    Vec4([t, -six, o, four]),
    Vec4([-t, t, t, l]),
    Vec4([l, o, o, o]),
  ]) * six.recip()
}
