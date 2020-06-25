use quick_maths::{Transform4, Vec3};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Builder {
  LookAt {
    origin: Vec3,
    towards: Vec3,
    up: Vec3,
  },
  Scale(Vec3),
  Rotate(Vec3, f32),
  Identity,
}

impl From<Builder> for Transform4 {
  fn from(tfb: Builder) -> Self {
    use Builder::*;
    match tfb {
      LookAt {
        origin,
        towards,
        up,
      } => Transform4::look_at(origin, towards, up),
      Scale(by) => Transform4::scale(by),
      Rotate(axis, theta) => Transform4::rot(axis, theta),
      Identity => Transform4::identity(),
    }
  }
}
