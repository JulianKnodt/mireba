use crate::vec::{Quat, Vec2, Vec3, Vec4, Vector};
use num::{Float, One, Zero};

/// A 3x3 Matrix type
// each vector represents a column
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix3<T>([Vec3<T>; 3]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix2<T>([Vec2<T>; 2]);

impl<T: Float> Matrix2<T> {
  pub fn vecmul(&self, v: &Vec2<T>) -> Vec2<T> {
    let &Matrix2([c0, c1]) = self;
    c0 * v.0 + c1 * v.1
  }
  pub fn matmul(&self, o: &Self) -> Self {
    let &Matrix2([c0, c1]) = o;
    Matrix2([self.vecmul(&c0), self.vecmul(&c1)])
  }
  /// Returns the rotation matrix given a theta in the counterclockwise direction
  pub fn rot(theta: T) -> Self {
    let (sin_t, cos_t) = theta.sin_cos();
    Matrix2([Vec2(cos_t, sin_t), Vec2(-sin_t, cos_t)])
  }
  /// Returns the scale matrix given scale in each direction
  pub fn scale(sx: T, sy: T) -> Self {
    let o = T::zero();
    Matrix2([Vec2(sx, o), Vec2(o, sy)])
  }
}

impl<T: Clone> From<T> for Matrix3<T> {
  fn from(t: T) -> Self { Matrix3([Vec3::from(t.clone()), Vec3::from(t.clone()), Vec3::from(t)]) }
}

impl<T: Zero> Zero for Matrix3<T> {
  fn zero() -> Self { Self([Vec3::zero(), Vec3::zero(), Vec3::zero()]) }
  fn is_zero(&self) -> bool { self.0.iter().all(|col| col.is_zero()) }
}

/// Multiplicative identity
impl<T: One + Zero + Copy + PartialEq> One for Matrix3<T> {
  fn one() -> Self {
    let o = T::zero();
    let l = T::one();
    Self([Vec3(l, o, o), Vec3(o, l, o), Vec3(o, o, l)])
  }
  fn is_one(&self) -> bool {
    let o = T::zero();
    let l = T::one();
    self == &Matrix3([Vec3(l, o, o), Vec3(o, l, o), Vec3(o, o, l)])
  }
}

impl<T: Float> From<Quat<T>> for Matrix3<T> {
  // https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation
  /// Converts a quaternion into an equivalent matrix
  fn from(q: Quat<T>) -> Self {
    let Quat(Vec3(x, y, z), w) = q;
    let t = T::from(2.0).unwrap();
    Matrix3([
      Vec3(
        w * w + x * x - y * y - z * z,
        t * x * y + t * w * z,
        t * x * z - t * w * y,
      ),
      Vec3(
        t * x * y - t * w * z,
        w * w - x * x + y * y - z * z,
        t * y * z + t * w * x,
      ),
      Vec3(
        t * x * z + t * w * y,
        t * y * z - t * w * x,
        w * w - x * x - y * y + z * z,
      ),
    ])
  }
}

impl<T: Float> Matrix3<T> {
  /// Computes the determinant of this matrix
  pub fn det(&self) -> T {
    let &Matrix3([Vec3(e00, e01, e02), Vec3(e10, e11, e12), Vec3(e20, e21, e22)]) = self;
    e00 * e11 * e22 +
    e01 * e12 * e20 +
    e02 * e10 * e21 -
    // subtraction
    e02 * e11 * e20 -
    e01 * e10 * e22 -
    e00 * e12 * e21
  }
  /// Basic vector multiplication by a vector
  pub fn vecmul(&self, o: &Vec3<T>) -> Vec3<T> {
    let &Matrix3([c0, c1, c2]) = self;
    let &Vec3(x, y, z) = o;
    c0 * x + c1 * y + c2 * z
  }
  /// returns the matrix multiplication of this matrix and another one
  pub fn matmul(&self, o: &Self) -> Self {
    let &Matrix3([c0, c1, c2]) = o;
    Matrix3([self.vecmul(&c0), self.vecmul(&c1), self.vecmul(&c2)])
  }
  /// Returns the main diagonal along this matrix
  pub fn diag(&self) -> Vec3<T> {
    let &Matrix3([Vec3(e00, _, _), Vec3(_, e11, _), Vec3(_, _, e22)]) = self;
    Vec3(e00, e11, e22)
  }
  /// returns the trace of this matrix
  pub fn trace(&self) -> T {
    let Vec3(e00, e11, e22) = self.diag();
    e00 + e11 + e22
  }
  /// transposes the matrix
  pub fn t(&self) -> Self {
    let &Matrix3([Vec3(e00, e01, e02), Vec3(e10, e11, e12), Vec3(e20, e21, e22)]) = self;
    Matrix3([
      Vec3(e00, e10, e20),
      Vec3(e01, e11, e21),
      Vec3(e02, e12, e22),
    ])
  }
  pub fn rot(around: &Vec3<T>, cos_t: T) -> Self {
    let &Vec3(i, j, k) = around;
    let l = T::one();
    let sin_t = l - cos_t * cos_t;
    Self([
      Vec3(
        i * i * (l - cos_t) + cos_t,
        i * j * (l - cos_t) + k * sin_t,
        i * k * (l - cos_t) - j * sin_t,
      ),
      Vec3(
        i * j * (l - cos_t) - k * sin_t,
        j * j * (l - cos_t) + cos_t,
        j * k * (l - cos_t) - i * sin_t,
      ),
      Vec3(
        i * k * (l - cos_t) + k * sin_t,
        j * k * (l - cos_t) - i * sin_t,
        k * k * (l - cos_t) + cos_t,
      ),
    ])
  }
  pub fn scale(by: &Vec3<T>) -> Self {
    let &Vec3(i, j, k) = by;
    let o = T::zero();
    Self([Vec3(i, o, o), Vec3(o, j, o), Vec3(o, o, k)])
  }
  /// translates x and y
  pub fn trans(by: &Vec2<T>) -> Self {
    let &Vec2(i, j) = by;
    let o = T::zero();
    let l = T::one();
    Self([Vec3(l, o, i), Vec3(o, l, j), Vec3(o, o, l)])
  }
  /// Projects onto the plane defined by normal
  pub fn project(normal: &Vec3<T>) -> Self {
    let normal = normal.norm();
    let Vec3(i, j, k) = normal;
    let l = T::one();
    let o = T::zero();
    let b_0 = match (i.is_zero(), j.is_zero(), k.is_zero()) {
      (true, true, true) => return Self::zero(),
      (true, true, false) | (true, false, true) => Vec3(l, o, o),
      (false, true, true) => Vec3(o, o, l),
      (false, false, true) => Vec3(-j, i, k),
      (false, true, false) => Vec3(-k, j, i),
      (true, false, false) => Vec3(i, -k, j),
      (false, false, false) => Vec3(i, k, -j),
    };
    let b_1 = normal.cross(&b_0);
    Self([b_0, b_1, Vec3::zero()])
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Matrix4<T>([Vec4<T>; 4]);

impl<T: Zero + Copy> Zero for Matrix4<T> {
  fn zero() -> Self { Self([Vec4::zero(); 4]) }
  fn is_zero(&self) -> bool { self.0.iter().all(|col| col.is_zero()) }
}

/// Multiplicative identity
impl<T: One + Zero + Copy + PartialEq> One for Matrix4<T> {
  fn one() -> Self {
    Self([
      Vec4::basis(0),
      Vec4::basis(1),
      Vec4::basis(2),
      Vec4::basis(3),
    ])
  }
  fn is_one(&self) -> bool {
    let l = T::one();
    let o = T::zero();
    &Matrix4([
      Vec4([l, o, o, o]),
      Vec4([o, l, o, o]),
      Vec4([o, o, l, o]),
      Vec4([o, o, o, l]),
    ]) == self
  }
}

impl<T: Float> Matrix4<T> {
  pub fn vecmul(&self, o: &Vec4<T>) -> Vec4<T> {
    let &Matrix4([c1, c2, c3, c4]) = self;
    let &Vec4([a, b, c, d]) = o;
    c1 * a + c2 * b + c3 * c + c4 * d
  }
  /// Implement matrix multiplication
  pub fn matmul(&self, o: &Self) -> Self {
    let Matrix4([a, b, c, d]) = o;
    Matrix4([
      self.vecmul(a),
      self.vecmul(b),
      self.vecmul(c),
      self.vecmul(d),
    ])
  }
  /// Returns a translation matrix by t
  pub fn translate(t: Vec3<T>) -> Self {
    let mut out = Matrix4::one();
    out.0[3] = t.into();
    out
  }
}

/// Extends a 3x3 matrix such that it has a 4th column with last element 1.
impl<T: One + Zero> From<Matrix3<T>> for Matrix4<T> {
  fn from(v: Matrix3<T>) -> Self {
    let Matrix3([a, b, c]) = v;
    Matrix4([
      a.extend(T::zero()),
      b.extend(T::zero()),
      c.extend(T::zero()),
      Vec4::basis(3),
    ])
  }
}

/*
// TODO look into using constant generics because this could replace all the code from above
// more easily maybe.
pub struct GenMatrix<T, const R: usize, const C: usize> {
  data: [[T; R]; C]
}

impl<T, const R: usize, const C: usize> GenMatrix {

}
*/

// Defines all element-wise operations between matrices of the same size
macro_rules! def_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T: $name> $name for Matrix2<T> {
      type Output = Matrix2<<T as $name>::Output>;
      fn $fn_name(self, o: Self) -> Self::Output {
        let Matrix2([a, b]) = self;
        let Matrix2([i, j]) = o;
        Matrix2([a $op i, b $op j])
      }
    }
    impl<T: $name> $name for Matrix3<T> {
      type Output = Matrix3<<T as $name>::Output>;
      fn $fn_name(self, o: Self) -> Self::Output {
        let Matrix3([a, b, c]) = self;
        let Matrix3([i, j, k]) = o;
        Matrix3([a $op i, b $op j, c $op k])
      }
    }
    impl<T: $name> $name for Matrix4<T> {
      type Output = Matrix4<<T as $name>::Output>;
      fn $fn_name(self, o: Self) -> Self::Output {
        let Matrix4([a, b, c, d]) = self;
        let Matrix4([i, j, k, l]) = o;
        Matrix4([a $op i, b $op j, c $op k, d $op l])
      }
    }
  }
}

// defines scalar operations between matrices
macro_rules! def_scalar_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T>$name<T> for Matrix2<T> where T: $name + Copy {
      type Output = Matrix2<<T as $name>::Output>;
      fn $fn_name(self, o: T) -> Self::Output {
        let Matrix2([a, b]) = self;
        Matrix2([a $op o, b $op o])
      }
    }
    impl<T>$name<T> for Matrix3<T> where T: $name + Copy {
      type Output = Matrix3<<T as $name>::Output>;
      fn $fn_name(self, o: T) -> Self::Output {
        let Matrix3([a, b, c]) = self;
        Matrix3([a $op o, b $op o, c $op o])
      }
    }
    impl<T>$name<T> for Matrix4<T> where T: $name + Copy {
      type Output = Matrix4<<T as $name>::Output>;
      fn $fn_name(self, o: T) -> Self::Output {
        let Matrix4([a, b, c, d]) = self;
        Matrix4([a $op o, b $op o, c $op o, d $op o])
      }
    }
  };
}

use std::ops::{Add, Div, Mul, Sub};
def_op!(Add, add, +);
def_op!(Sub, sub, -);
def_op!(Mul, mul, *);
def_op!(Div, div, /);

def_scalar_op!(Add, add, +);
def_scalar_op!(Sub, sub, -);
def_scalar_op!(Mul, mul, *);
def_scalar_op!(Div, div, /);
