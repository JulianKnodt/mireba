use crate::renderable::Storage;

/// A serialized format for storing in a file
#[derive(Debug, Clone)]
pub struct Object<M, S> {
  pub mat: M,
  pub shape: S,
}

impl<M, S> Object<M, S> {
  pub fn new(shape: S, mat: M) -> Self { Self { shape, mat } }
}

impl Object<usize, usize> {
  pub fn resolve<'m, 's, M, S>(self, ms: &'m [M], ss: &'s [S]) -> Object<&'m M, &'s S> {
    let Object { mat, shape } = self;
    Object {
      mat: &ms[mat],
      shape: &ss[shape],
    }
  }
}

impl<'m, 's, M: PartialEq, S: PartialEq> Object<&'m M, &'s S> {
  /// Attempts to dissolve this object into a indexed reference
  pub fn dissolve(self, mats: &'m [M], shapes: &'s [S]) -> Option<Object<usize, usize>> {
    let Object { mat, shape } = self;
    let mat_pos = mats.iter().position(|m| m == mat)?;
    let shape_pos = shapes.iter().position(|s| s == shape)?;
    Some(Object {
      mat: mat_pos,
      shape: shape_pos,
    })
  }
}

impl From<Object<usize, usize>> for Storage {
  fn from(o: Object<usize, usize>) -> Self { Storage::Shape(o) }
}
