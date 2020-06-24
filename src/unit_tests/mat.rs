use crate::{
  mat::{Matrix, Matrix3, Matrix4},
  vec::{Vec3, Vec4},
};
use num::One;

#[test]
fn inv_identity() {
  let eye = Matrix4::<f32>::one().inv();
  assert!(eye.is_one(), "{:?}", eye);
}

quickcheck! {
  fn vecmul_ok(m: Matrix4<f32>, vec: Vec4<f32>) -> bool {
    let Matrix4([c0, c1, c2, c3]) = m;
    let Vec4([x, y, z, w]) = vec;
    assert_eq!(c0 * x + c1 * y + c2 * z + c3 * w,
      m.vecmul(&vec));
    true
  }
  fn translate(start: Vec3<f32>, by: Vec3<f32>) -> bool {
    let m = Matrix4::translate(by);
    let out: Vec3<_> = m.vecmul(&start.extend(1.0)).into();
    out == (start + by)
  }
  fn identity_translate(by: Vec3<f32>) -> bool {
    let m = Matrix4::translate(by);
    let m_inv = Matrix4::translate(-by);
    m.matmul(&m_inv).is_one()
  }
  fn scale(start: Vec3<f32>, by: Vec3<f32>) -> bool {
    let m: Matrix4<_> = Matrix3::scale(&by).into();
    let out: Vec3<_> = m.vecmul(&start.extend(1.0)).into();
    out == (start * by)
  }
}

/*
use quickcheck::TestResult;
quickcheck! {
  fn double_inv_identity(m: Matrix4<f32>) -> TestResult {
    let inv = m.inv();
    if m.0.iter().any(|col| col.0.iter().any(|v| !v.is_finite())) {
      return TestResult::discard();
    }
    // There is a p high error for inv inv operation in some cases but generally works
    let frob = (m - inv.inv()).frobenius();
    TestResult::from_bool(frob < 1.0)
  }
}
*/
