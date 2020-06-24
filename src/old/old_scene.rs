use linalg::vec::{Ray, Vec3};
use crate::{
  light::Light,
  mtl::MTL,
  num::Float,
  object::Object,
  renderable::{Renderable, Shapes, Storage},
};
use rand::prelude::*;
use rand_distr::{Standard, StandardNormal};

#[derive(Debug, Copy, Clone, Default)]
pub struct GlobalSettings<D> {
  recursive_depth: u16,
  pub ambient_illumination: Vec3<D>,
  background_color: Vec3<D>,
  // TODO need to specify a default brdf
  // default_brdf: BRDF,
}

impl<D: Float> GlobalSettings<D> {
  pub fn with_ambience(self, ambient_illumination: Vec3<D>) -> Self {
    Self {
      ambient_illumination,
      ..self
    }
  }
  pub fn with_background(self, background_color: Vec3<D>) -> Self {
    Self {
      background_color,
      ..self
    }
  }
}

#[derive(Debug, Default)]
pub struct Scene<D: Float> {
  // TODO include camera here
  pub materials: Vec<MTL<D>>,
  pub shapes: Vec<Shapes<D>>,
  pub objects: Vec<Storage>,
  pub lights: Vec<Light<D>>,

  pub settings: GlobalSettings<D>,
}

/// A scene that is ready to render
#[derive(Debug)]
pub struct ReadyScene<'f, 'm, 's, D: Float> {
  pub objects: Vec<Renderable<'m, 's, D>>,
  pub lights: &'f [Light<D>],

  pub settings: &'f GlobalSettings<D>,
}

impl<D: Float> Scene<D> {
  pub fn new(
    materials: Vec<MTL<D>>,
    shapes: Vec<Shapes<D>>,
    objects: Vec<Storage>,
    lights: Vec<Light<D>>,
    settings: GlobalSettings<D>,
  ) -> Self {
    Self {
      materials,
      shapes,
      objects,
      lights,
      settings,
    }
  }
}

impl<D: Float> Scene<D> {
  pub fn ready(&self) -> ReadyScene<'_, '_, '_, D> {
    let objects = self
      .objects
      .iter()
      .cloned()
      .map(|stored| stored.resolve(&self.materials[..], &self.shapes[..]))
      .collect::<Vec<_>>();
    let Scene {
      lights, settings, ..
    } = self;
    ReadyScene {
      objects,
      lights: &lights[..],
      settings: &settings,
    }
  }
}

impl<D: Float> ReadyScene<'_, '_, '_, D>
where
  StandardNormal: Distribution<D>,
  Standard: Distribution<D>,
{
  pub fn color_at(&self, r: &Ray<D>) -> Vec3<D> {
    if let Some((vis, mat)) = crate::vis::trace_ray(r, self.objects.iter()) {
      (mat.illum.brdf())(r, &mat, &vis, self)
    } else {
      self.settings.background_color
    }
  }
}

impl<D: Float> ReadyScene<'_, '_, '_, D> {
  pub fn ambient_illumination(&self) -> Vec3<D> { self.settings.ambient_illumination }
}

/// Takes a set of objects and rehydrates them from slices
pub fn resolve_objects<'v, 'm, 's, M, S>(
  ms: &'m [M],
  ss: &'s [S],
  objects: Vec<Object<usize, usize>>,
) -> impl Iterator<Item = Object<&'m M, &'s S>> + 'v
where
  'm: 'v,
  's: 'v, {
  objects.into_iter().map(move |o| o.resolve::<M, S>(&ms, ss))
}

/// Takes a set of objects and their sources and decomposes them into indeces
pub fn dissolve_objects<'v, 'm, 's, M: PartialEq, S: PartialEq>(
  ms: &'m [M],
  ss: &'s [S],
  objects: Vec<Object<&'m M, &'s S>>,
) -> impl Iterator<Item = Object<usize, usize>> + 'v
where
  'm: 'v,
  's: 'v, {
  objects.into_iter().filter_map(move |o| o.dissolve(ms, ss))
}
