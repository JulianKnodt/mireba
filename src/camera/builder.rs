use super::{
  orthographic::Orthographic, perspective::Perspective, Cameras, Variant as CameraVariant,
};
use quick_maths::Transform4;

/// Camera Builder
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Builder {
  pub film_builder: crate::film::builder::Builder,
  pub to_world: crate::transform::Builder,
  pub variant: Variant,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Variant {
  Perspective {
    x_fov: f32,
    near_clip: f32,
    far_clip: f32,
    aspect: f32,
  },

  Orthographic {
    near_clip: f32,
    far_clip: f32,
    aspect: f32,
  },
}

impl From<Builder> for Cameras {
  fn from(b: Builder) -> Self {
    let Builder {
      film_builder,
      to_world,
      variant,
    } = b;
    let to_world: Transform4 = to_world.into();
    let variant = match variant {
      Variant::Perspective {
        x_fov,
        near_clip,
        far_clip,
        aspect,
      } => CameraVariant::Perspective(Perspective::new(x_fov, near_clip, far_clip, aspect)),
      Variant::Orthographic {
        near_clip,
        far_clip,
        aspect,
      } => CameraVariant::Orthographic(Orthographic::new(near_clip, far_clip, aspect)),
    };
    Cameras {
      from_world: to_world.inv(),
      to_world,
      film: film_builder.into(),
      variant,
    }
  }
}
