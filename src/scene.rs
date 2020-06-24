use crate::{
  accelerator::Accelerator,
  bsdf::BSDFImpl,
  camera::Cameras,
  integrator::Integrator,
  interaction::SurfaceInteraction,
  light::Lights,
  shapes::{ShapeImpl, Shapes},
};
use quick_maths::Ray;
use std::collections::HashMap;

// TODO add Serde for RawScene
#[derive(Debug, serde::Deserialize)]
pub struct RawScene {
  /// List of lights
  lights: Vec<Lights>,
  /// Camera
  camera: Cameras,
  /// List of shapes with optional ids
  shapes: HashMap<String, ShapeImpl>,
  /// List of BSDFs with optional ids
  bsdfs: HashMap<String, BSDFImpl>,
  /// Mapping between shapes -> bsdf
  bsdf_mapping: HashMap<String, String>,
}

impl RawScene {
  /// Create an acceleration structure from a raw scene
  pub fn build<El, Acc: Accelerator>(self) -> Scene<El, Acc> {
    let RawScene {
      lights,
      camera,
      shapes,
      bsdfs,
      bsdf_mapping,
    } = self;
    let (id_to_idx, mut bsdfs): (HashMap<_, _>, Vec<_>) = bsdfs
      .into_iter()
      .enumerate()
      .map(|(i, (id, v))| ((id, i), v))
      .unzip();
    let shapes = shapes.into_iter().map(|(shape_id, shape_impl)| {
      let idx = id_to_idx[&bsdf_mapping[&shape_id]];
      Shapes::new(shape_impl, &mut bsdfs[idx])
    });
    Scene {
      lights,
      camera,
      env_light: None,
      accelerator: Acc::build(shapes),
      bsdfs,
    }
  }
}

#[derive(Debug)]
pub struct Scene<EnvLight, Acc> {
  /// List of lights for this scene
  pub lights: Vec<Lights>,

  /// Camera for this scene
  pub camera: Cameras,

  /// Environment light
  pub env_light: Option<EnvLight>,

  /// List of BSDFs used in this implementation
  bsdfs: Vec<BSDFImpl>,

  /// Acceleration data structure
  accelerator: Acc,
}

impl<El, Acc> Scene<El, Acc>
where
  Acc: Accelerator,
{
  // TODO build something that creates a scene from iterators of shapes
  // pub fn new(items: Vec<Lights>, ...)
  pub fn render<I: Integrator>(&self, int: I) { int.render(self); }
  pub fn intersect_ray(&self, r: &Ray) -> Option<(SurfaceInteraction, &Shapes)> {
    self.accelerator.intersect_ray(r)
  }
}
