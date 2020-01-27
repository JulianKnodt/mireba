use crate::{
  bounds::Bounds,
  material::{Checkers, MatRef},
  plane::Plane,
  sphere::Sphere,
  vec::{Ray, Vec3},
};
use quickcheck::{Arbitrary, Gen};

/*
Some boiler plate for using quick check.
*/

const TEST_MAT: Checkers = Checkers {};
const TEST_MATREF: MatRef<'static, f32> = MatRef::Checkers(&TEST_MAT);

impl Arbitrary for Vec3<f32> {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    Vec3(f32::arbitrary(g), f32::arbitrary(g), f32::arbitrary(g))
  }
}

impl Arbitrary for Ray<f32> {
  fn arbitrary<G: Gen>(g: &mut G) -> Self { Ray::new(Vec3::arbitrary(g), Vec3::arbitrary(g)) }
}

impl Arbitrary for Sphere<'static, f32> {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    Sphere::new(Vec3::arbitrary(g), f32::arbitrary(g).abs(), TEST_MATREF)
  }
}

impl Arbitrary for Bounds<f32> {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    Bounds::valid([Vec3::arbitrary(g), Vec3::arbitrary(g)])
  }
}

impl Arbitrary for Plane<'static, f32> {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    Plane::new(Vec3::arbitrary(g), f32::arbitrary(g), TEST_MATREF)
  }
}
