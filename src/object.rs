use serde::{Deserialize, Serialize};

/// A shape, material tuple for rendering
#[derive(Debug, Serialize, Deserialize)]
pub struct Object<M, S> {
  pub mat: M,
  pub shape: S,
}

impl<M, S> Object<M, S> {
  pub fn new(shape: S, mat: M) -> Self { Self { shape, mat } }
}

impl<S> Object<usize, S> {
  pub fn resolve<M>(self, ms: &[M]) -> Object<&'_ M, S> {
    let Object { mat, shape } = self;
    Object {
      mat: &ms[mat],
      shape,
    }
  }
}

impl<'a, M: PartialEq, S> Object<&'a M, S> {
  /// Attempts to dissolve this object into a indexed reference
  pub fn dissolve(self, src: &'a [M]) -> Option<Object<usize, S>> {
    let Object { mat, shape } = self;
    let pos = src.iter().position(|m| m == mat)?;
    Some(Object { mat: pos, shape })
  }
}
