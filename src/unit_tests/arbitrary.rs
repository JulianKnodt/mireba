use crate::{
  bounds::Bounds3,
  shapes::{sphere::Sphere, triangle::Triangle},
};
use quick_maths::Vec3;
use quickcheck::{Arbitrary, Gen};

/*
Some boiler plate for using quick check.
*/

impl Arbitrary for Bounds3 {
  fn arbitrary<G: Gen>(g: &mut G) -> Self { Bounds3::new(Vec3::arbitrary(g), Vec3::arbitrary(g)) }
}

impl Arbitrary for Sphere {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    Sphere::new(Vec3::arbitrary(g), f32::arbitrary(g).abs())
  }
}

/*
impl Arbitrary for Plane {
  fn arbitrary<G: Gen>(g: &mut G) -> Self { Plane::new(Vec3::arbitrary(g), f32::arbitrary(g)) }
}
*/

impl Arbitrary for Triangle {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    let v0 = Vec3::arbitrary(g);
    let v1 = loop {
      let choice = Vec3::arbitrary(g);
      if choice != v0 {
        break choice;
      }
    };
    loop {
      let v2 = Vec3::arbitrary(g);
      let t = Triangle(Vec3::new(v0, v1, v2));
      if t.area() > std::f32::EPSILON * 3.0 {
        break t;
      }
    }
  }
}
