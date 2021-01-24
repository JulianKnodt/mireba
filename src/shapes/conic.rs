use crate::vec::Ray3;

// General conic sections need to implement actual intersection but will implement that later

/// A Cylinder
#[derive(Debug, Clone)]
pub struct Cylinder<T> {
  /// Where this cylinder is and how is it oriented?
  /// Length of the direction determines its height
  loc: Ray3<T>,
  /// how large is the base
  base_radius: T,
}

/// A Cone
#[derive(Debug, Clone)]
pub struct Cone<T> {
  loc: Ray3<T>
  base_radius: T,
}

