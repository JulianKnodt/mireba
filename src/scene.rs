use crate::{
  material::Mat,
  light::PointLight,
  renderable::Renderable,
};


/// A set of items which are being rendered by the scene.
#[derive(Debug, Default)]
pub struct Scene {
  materials: Vec<Mat>,
  objects: Vec<Renderable>,
  lights: Vec<PointLight>,
}

