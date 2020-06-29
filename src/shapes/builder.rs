use super::Geometry;
use quick_maths::Vec3;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Builder {
  pub to_world: crate::transform::Builder,
  pub variant: Variant,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Variant {
  Sphere {
    center: Vec3,
    radius: f32,
  },
  Plane {
    normal: Vec3,
    w: f32,
    up: Vec3,
    width: f32,
    height: f32,
  },
  Triangle(Vec3<Vec3>),
  Obj {
    file: String,
  },
}

impl From<Builder> for Geometry {
  fn from(b: Builder) -> Self {
    let Builder { to_world, variant } = b;
    use super::Variant as GeoVariant;
    use Variant::*;
    let variant = match variant {
      Sphere { center, radius } => GeoVariant::Sphere(super::sphere::Sphere::new(center, radius)),
      Plane {
        normal,
        w,
        up,
        width,
        height,
      } => GeoVariant::Plane(super::plane::Plane::new(&normal, w, &up, width, height)),
      Triangle(verts) => GeoVariant::Triangle(super::triangle::Triangle(verts)),
      Obj { file: _file } => todo!(),
    };
    Self {
      to_world: to_world.into(),
      variant,
    }
  }
}
