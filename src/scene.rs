use crate::{
  accelerator::Accelerator,
  bsdf::{builder::Builder as BSDFBuilder, BSDFImpl},
  camera::{builder::Builder as CameraBuilder, Cameras},
  integrator::Integrator,
  interaction::SurfaceInteraction,
  light::Lights,
  shapes::{Builder as ShapeBuilder, Shapes},
  spectrum::from_rgb,
  transform::Builder as TransformBuilder,
};
use quick_maths::{Ray, Vec3};
use std::collections::HashMap;

// TODO add Serde for RawScene
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RawScene {
  /// List of lights
  lights: Vec<Lights>,
  /// Camera
  camera: CameraBuilder,
  /// List of shapes with optional ids
  shapes: HashMap<String, ShapeBuilder>,
  /// List of BSDFs with optional ids
  bsdfs: HashMap<String, BSDFBuilder>,
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
      .map(|(i, (id, v))| ((id, i), v.into()))
      .unzip();
    let shapes = shapes.into_iter().map(|(shape_id, shape_builder)| {
      let idx = id_to_idx[&bsdf_mapping[&shape_id]];
      Shapes::new(shape_builder.into(), &mut bsdfs[idx])
    });
    Scene {
      lights,
      camera: camera.into(),
      env_light: None,
      accelerator: Acc::build(shapes),
      bsdfs,
    }
  }
  /// Creates an example
  pub fn example() -> Self {
    let lights = vec![Lights::Point(crate::light::point::Point::new(
      Vec3::new(0.0, 1.0, -1.0),
      10.0,
      from_rgb(Vec3::new(0.3, 0.7, 0.9)),
    ))];
    let mut shapes = HashMap::new();
    shapes.insert(String::from("central_sphere"), ShapeBuilder {
      to_world: TransformBuilder::Identity,
      variant: crate::shapes::builder::Variant::Sphere {
        center: Vec3::new(0., 0., 10.),
        radius: 1.0,
      },
    });
    let mut bsdfs = HashMap::new();
    bsdfs.insert(
      String::from("debug"),
      BSDFBuilder::Diffuse(from_rgb(Vec3::new(0.7, 0.8, 0.5))),
    );
    let mut bsdf_mapping = HashMap::new();
    bsdf_mapping.insert(String::from("central_sphere"), String::from("debug"));
    Self {
      lights,
      camera: CameraBuilder {
        film_builder: crate::film::builder::Builder { size: (512, 512) },
        to_world: TransformBuilder::LookAt {
          origin: Vec3::of(0.0),
          towards: Vec3::new(0., 0., 1.),
          up: Vec3::new(0., 1., 0.),
        },
        variant: crate::camera::builder::Variant::Perspective {
          x_fov: 30.0,
          near_clip: 1.0e-3,
          far_clip: 1.0e3,
          aspect: 1.0,
        },
        sampler: None,
      },
      // TODO fill in examples here
      shapes,
      bsdfs,
      bsdf_mapping,
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
