use super::{orthographic::Orthographic, perspective::Perspective, Cameras};
use quick_maths::Transform4;

/// Camera Builder
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Builder {
  pub film_builder: crate::film::builder::Builder,
  pub to_world: crate::transform::Builder,
  pub sampler: Option<crate::sampler::builder::Builder>,
  pub variant: Variant,
}

/// Builder Variants
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

impl From<Variant> for super::Variant {
  fn from(v: Variant) -> Self {
    match v {
      Variant::Perspective {
        x_fov,
        near_clip,
        far_clip,
        aspect,
      } => Self::Perspective(Perspective::new(x_fov, near_clip, far_clip, aspect)),
      Variant::Orthographic {
        near_clip,
        far_clip,
        aspect,
      } => Self::Orthographic(Orthographic::new(near_clip, far_clip, aspect)),
    }
  }
}

impl From<Builder> for Cameras {
  fn from(b: Builder) -> Self {
    let Builder {
      film_builder,
      to_world,
      variant,
      sampler,
    } = b;
    let sampler = sampler.unwrap_or_else(Default::default);
    let to_world: Transform4 = to_world.into();
    Cameras {
      from_world: to_world.inv(),
      to_world,
      film: film_builder.into(),
      variant: variant.into(),
      sampler: sampler.into(),
    }
  }
}
