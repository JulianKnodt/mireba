use super::Geometry;
use quick_maths::Vec3;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Builder {
  pub to_world: crate::transform::Builder,
  pub variant: Variant,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Variant {
  Sphere { center: Vec3, radius: f32 },
  Obj { file: String },
}

impl From<Builder> for Geometry {
  fn from(b: Builder) -> Self {
    let Builder { to_world, variant } = b;
    use Variant::*;
    let variant = match variant {
      Sphere { center, radius } =>
        super::Variant::Sphere(super::sphere::Sphere::new(center, radius)),
      Obj { file } => todo!(),
    };
    Self {
      to_world: to_world.into(),
      variant,
    }
  }
}
