use crate::triangle::Triangle;
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
  fn intersection(r: Ray<f32>, t: Triangle<Vec3<f32>>) -> TestResult {
    match t.as_ref().moller_trumbore(&r) {
      None => TestResult::discard(),
      Some(v) => TestResult::from_bool(t.as_ref().contains(&v.pos)),
    }
  }
}
