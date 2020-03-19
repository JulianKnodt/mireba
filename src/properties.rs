use crate::{map::Map, vec::*};
use crate::mat::Matrix3;
use quickcheck::TestResult;

quickcheck! {
  // tests that quaternion correctly maps into a matrix
  fn quat_eq_mat(src: Vec3<f32>, rot: Vec3<f32>) -> TestResult {
    let q = Quat::rot(&rot);
    let mat: Matrix3<f32> = q.into();
    TestResult::from_bool((src.apply(q) - src.apply(mat)).sqr_magn() < 0.001)
  }
}
