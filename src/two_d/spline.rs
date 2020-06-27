use quick_maths::{Float, Mat4, Matrix, Vec2, Vec3, Vec4, Vector};

/// A cubic spline
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct CubicSpline<const M: usize> {
  /// Control points for use in interpolation.
  ctrls: Matrix<f32, M, 4>,
}

fn cubic_comp<T: Float>(u: T) -> Vec4<T> {
  let u_2 = u * u;
  Vec4::new(u_2 * u, u_2, u, T::one())
}

/*
impl<V> CubicSpline<V> {
  pub fn new(ctrls: [V; 4]) -> Self {
    Self {
      ctrls,
    }
  }
}
*/

/*
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
*/

impl<const M: usize> CubicSpline<M> {
  pub fn components(&self) -> [Vec4; M] {
    let Matrix(Vector(v)) = self.ctrls.t();
    v
  }
}

impl CubicSpline<2> {
  pub fn at(&self, u: f32, weights: &Mat4) -> Vec2 {
    let us = cubic_comp(u.max(0.0).min(1.0));
    let [xs, ys] = self.components();
    let x = weights.dot(&xs).dot(&us);
    let y = weights.dot(&ys).dot(&us);
    Vec2::new(x, y)
  }
  pub fn sample<'a, 'b>(&'b self, n: u32, weights: &'a Mat4) -> impl Iterator<Item = Vec2> + 'b
  where
    'a: 'b, {
    let step = ((n - 1) as f32).recip();
    (0..n)
      .map(move |i| step * (i as f32))
      .map(move |u| self.at(u, &weights))
  }
}

impl CubicSpline<3> {
  pub fn at(&self, u: f32, weights: &Mat4) -> Vec3 {
    let us = cubic_comp(u.max(0.0).min(1.0));
    let [xs, ys, zs] = self.components();
    let x = weights.dot(&xs).dot(&us);
    let y = weights.dot(&ys).dot(&us);
    let z = weights.dot(&zs).dot(&us);
    Vec3::new(x, y, z)
  }
  /*
  pub fn surface(&self, (u, v): (f32, f32), weights: &Mat4) -> Vec3 {
    let us = cubic_comp(u.max(0.0).min(1.0));
    let vs = cubic_comp(v.max(0.0).min(1.0));
    let [xs, ys, zs] = self.components();
    let x = weights.dot(&xs).dot(&us);
    let y = weights.dot(&ys).dot(&us);
    let z = weights.dot(&zs).dot(&us);
    let partial = weights.t().dot(&vs);
    Vec3::new(
      weights.dot(&xs.dot(&partial)).dot(&us),
      weights.dot(&ys.dot(&partial)).dot(&us),
      weights.dot(&zs.dot(&partial)).dot(&us),
    )
  }
  */
  pub fn sample<'a, 'b>(&'b self, n: u32, weights: &'a Mat4) -> impl Iterator<Item = Vec3> + 'b
  where
    'a: 'b, {
    let step = ((n - 1) as f32).recip();
    (0..n)
      .map(move |i| step * (i as f32))
      .map(move |u| self.at(u, &weights))
  }
}

/*

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
    Vec3(
      weights.vecmul(&xs.vecmul(&partial)).dot(&us),
      weights.vecmul(&ys.vecmul(&partial)).dot(&us),
      weights.vecmul(&zs.vecmul(&partial)).dot(&us),
    )
  }
}

*/

pub const CUBIC_BEZIER_WEIGHTS: Mat4 = Matrix(Vector([
  Vector([-1., 3., -3., 1.]),
  Vector([3., -6., 3., 0.]),
  Vector([-3., 3., 0., 0.]),
  Vector([1., 0., 0., 0.]),
]));

pub const CUBIC_B_WEIGHTS: Mat4 = Matrix(Vector([
  Vector([-1., 3., -3., 1.]),
  Vector([3., -6., 0., 4.]),
  Vector([-3., 3., 3., 1.]),
  Vector([1., 0., 0., 0.]),
]));
