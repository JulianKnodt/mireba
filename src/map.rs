use crate::mat::{Matrix, Matrix2, Matrix3};
/// Defines computation and application of operators
/// over various vector types.
use crate::vec::{Quat, Vec2, Vec3, Vector};
use num::Float;

/// Computes the mapping on some type by some operator
pub trait Map<Op> {
  type Operator;
  /// Computes a rotation of self by the operator
  fn apply(&self, by: Op) -> Self;
  /// Computes an operator which maps self to the destination
  fn inverse(&self, dst: &Self) -> Op;
}

// chose to represent rotation instead of scaling because scaling is not hard to implement
// whereas scaling has a lot more moving components.

/// Rotation in radians counterclockwise
impl<D: Float> Map<D> for Vec2<D> {
  type Operator = D;
  /// Rotates a vector in 2 space by theta
  fn apply(&self, theta: Self::Operator) -> Self {
    use num::complex::Complex;
    let a = Complex::new(self.0, self.1);
    let rot = Complex::new(theta.cos(), theta.sin());
    let result = a * rot;
    Vec2(result.re, result.im)
  }
  fn inverse(&self, dst: &Self) -> Self::Operator { self.angle(dst) }
}

/// Rotation & scale over 2-space
impl<D: Float> Map<Matrix2<D>> for Vec2<D> {
  type Operator = Matrix2<D>;
  fn apply(&self, op: Matrix2<D>) -> Self { op.vecmul(self) }
  fn inverse(&self, o: &Self) -> Matrix2<D> {
    let factor = self.scale_ratio(o);
    Matrix2::rot(self.signed_angle(o)) * factor
  }
}

/// Scaling and rotation for 3-space
impl<D: Float> Map<Quat<D>> for Vec3<D> {
  type Operator = Quat<D>;
  fn apply(&self, op: Quat<D>) -> Self { self.apply_quat(&op) }
  fn inverse(&self, o: &Self) -> Quat<D> {
    let k_cos_theta = self.dot(&o);
    let k = (self.sqr_magn() * o.sqr_magn()).sqrt();
    if k_cos_theta == -k {
      todo!();
    }
    Quat(self.cross(&o), k + k_cos_theta)
      .norm()
      .scale(self.scale_ratio(o))
  }
}

/// Scaling and rotation for 3-space
impl<D: Float> Map<Matrix3<D>> for Vec3<D> {
  type Operator = Matrix3<D>;
  fn apply(&self, op: Self::Operator) -> Self { op.vecmul(&self) }
  fn inverse(&self, o: &Self) -> Self::Operator {
    let q: Quat<D> = self.inverse(o);
    q.into()
  }
}

#[cfg(test)]
mod map_tests {
  use super::Map;
  use crate::{
    mat::Matrix2,
    vec::{Quat, Vec2, Vec3, Vector},
  };
  use quickcheck::TestResult;
  #[test]
  fn test_map_vec2() {
    use num::abs;
    use std::f32::consts::PI;
    let v = Vec2(1.0, 0.0);
    let Vec2(x, y) = v.apply(PI / 2f32);
    let eps = 0.00001;
    assert!(abs(x) < eps);
    assert!(abs(y - 1.0) < eps);
    let Vec2(x, y) = v.apply(Matrix2::rot(5.0 * PI / 2.0));
    assert!(abs(x) < eps, "X did not match(0.0): {:?}", Vec2(x, y));
    assert!(abs(y - 1.0) < eps, "Y did not match(1.0): {:?}", Vec2(x, y));
  }
  quickcheck! {
    // checking that operation applied
    fn map_op_3d_identity(src: Vec3<f32>, dst: Vec3<f32>) -> TestResult {
      let op: Quat<_> = src.inverse(&dst);
      let applied = src.apply(op);
      TestResult::from_bool((dst - applied).sqr_magn() < 0.00001)
    }
  }
  quickcheck! {
    // checking that operation applied
    fn map_op_2d_identity(src: Vec2<f32>, dst: Vec2<f32>) -> TestResult {
      let op: Matrix2<f32> = src.inverse(&dst);
      let applied = src.apply(op);
      TestResult::from_bool((dst - applied).sqr_magn() < 0.00001)
    }
  }
}
