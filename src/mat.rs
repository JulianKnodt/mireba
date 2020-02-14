use crate::vec::Vec3;
use num::{Float, One, Zero};

/// A 3x3 Matrix type represented with vectors internally
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Matrix3<T>([Vec3<T>; 3]);

impl<T: Clone> From<T> for Matrix3<T> {
  fn from(t: T) -> Self { Matrix3([Vec3::from(t.clone()), Vec3::from(t.clone()), Vec3::from(t)]) }
}

impl<T: Zero + Copy> Zero for Matrix3<T> {
  fn zero() -> Self { Self([Vec3::zero(); 3]) }
  fn is_zero(&self) -> bool { self.0.iter().all(|col| col.is_zero()) }
}

impl<T: One + Zero + Copy + PartialEq> One for Matrix3<T> {
  fn one() -> Self {
    Self([
      Vec3(T::one(), T::zero(), T::zero()),
      Vec3(T::zero(), T::one(), T::zero()),
      Vec3(T::zero(), T::zero(), T::one()),
    ])
  }
  fn is_one(&self) -> bool {
    let &Matrix3([Vec3(e00, e01, e02), Vec3(e10, e11, e12), Vec3(e20, e21, e22)]) = self;
    e00.is_one()
      && e11.is_one()
      && e22.is_one()
      && e01.is_zero()
      && e02.is_zero()
      && e10.is_zero()
      && e20.is_zero()
      && e21.is_zero() & e12.is_zero()
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
    self.0[0] * o.0 + self.0[1] * o.1 + self.0[2] * o.2
  }
  /// returns the matrix multiplication of this matrix and another one
  pub fn matmul(&self, o: &Self) -> Self {
    let &Matrix3([c0, c1, c2]) = o;
    Matrix3([self.vecmul(&c0), self.vecmul(&c1), self.vecmul(&c2)])
  }
  /// returns the trace of this matrix
  pub fn trace(&self) -> T {
    let &Matrix3([Vec3(e00, _, _), Vec3(_, e11, _), Vec3(_, _, e22)]) = self;
    e00 + e11 + e22
  }
  pub fn rot(around: &Vec3<T>, cos_t: T) -> Self {
    let &Vec3(i, j, k) = around;
    let one = T::one();
    let sin_t = one - cos_t * cos_t;
    Self([
      Vec3(
        i * i * (one - cos_t) + cos_t,
        i * j * (one - cos_t) + k * sin_t,
        i * k * (one - cos_t) - j * sin_t,
      ),
      Vec3(
        i * j * (one - cos_t) - k * sin_t,
        j * j * (one - cos_t) + cos_t,
        j * k * (one - cos_t) - i * sin_t,
      ),
      Vec3(
        i * k * (one - cos_t) + k * sin_t,
        j * k * (one - cos_t) - i * sin_t,
        k * k * (one - cos_t) + cos_t,
      ),
    ])
  }
}

// Defines all element-wise operations
macro_rules! def_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T: $name + Copy> $name for Matrix3<T> {
      type Output = Matrix3<<T as $name>::Output>;
        fn $fn_name(self, o: Self) -> Self::Output {
          Matrix3([self.0[0] $op o.0[1], self.0[1] $op o.0[1], self.0[2] $op o.0[2]])
        }
      }
    }
}

use std::ops::{Add, Div, Mul, Sub};
def_op!(Mul, mul, *);
def_op!(Div, div, /);
def_op!(Add, add, +);
def_op!(Sub, sub, -);
