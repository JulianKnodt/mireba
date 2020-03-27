use crate::{light::Light, material::Mat, object::Object};
use serde::{Deserialize, Serialize};

/// Serializable and deserializable scene intended for scene construction.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Scene<D, S> {
  pub materials: Vec<Mat<D>>,
  pub objects: Vec<Object<usize, S>>,
  pub lights: Vec<Light<D>>,
}
// TODO might need to add some sort of builder pattern here which allows for taking the objects
// first then taking the materials

impl<D: PartialEq, S> Scene<D, S> {
  pub fn new(
    materials: Vec<Mat<D>>,
    objects: Vec<Object<&Mat<D>, S>>,
    lights: Vec<Light<D>>,
  ) -> Self {
    let objects = dissolve_objects(&materials[..], objects).collect::<Vec<_>>();
    Self {
      materials,
      objects,
      lights,
    }
  }
}

pub fn resolve_objects<M, S>(
  ms: &[M],
  objects: Vec<Object<usize, S>>,
) -> impl Iterator<Item = Object<&'_ M, S>> {
  objects.into_iter().map(move |o| o.resolve::<M>(&ms))
}

pub fn dissolve_objects<'v, M: PartialEq, S: 'v>(
  ms: &'v [M],
  objects: Vec<Object<&'v M, S>>,
) -> impl Iterator<Item = Object<usize, S>> + 'v {
  objects.into_iter().filter_map(move |o| o.dissolve(ms))
}
