use crate::bounds::{Bounded, Bounds};
use num::Float;

const ROOT: usize = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Region {
  FFF = 0,
  FFB = 1,
  FBF = 2,
  BFF = 3,
  FBB = 4,
  BFB = 5,
  BBF = 6,
  BBB = 7,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Octant<D> {
  bounds: Bounds<D>,
  // Don't think I need a parent for this struct
  /// Where in the original Octree are the children of this octant
  /// There will always be only 8.
  children: Option<[usize; 8]>,

  // TODO convert this into some sort of slice?
  /// Items inside this octant
  items: Vec<usize>,
}

impl<D> Octant<D> {
  fn new(bounds: Bounds<D>) -> Self {
    Self {
      bounds,
      children: None,
      items: vec![],
    }
  }
  #[inline]
  fn has_children(&self) -> bool {
    self.children.is_some()
  }
}

impl<D: Float> Octant<D> {
  /// Returns this octant's if it needs to have children created for it
  /// Otherwise returns None
  fn push<B: Bounded<D>>(&mut self, idx: usize, b: Bounds<D>) -> Option<usize> {
    assert!(self.bounds.contains(b));
    let vol = b.volume();
    if vol > self.bounds.volume()/T::from(8.0).unwrap() {
      //
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Octree<D, B: Bounded<D>> {
  items: Vec<B>,
  nodes: Vec<Octant<D>>,
}

impl<D: Float, B: Bounded<D>> Octree<D, B> {
  pub fn new(total_bounds: Bounds<D>) -> Self {
    let root = Octant::new(total_bounds);
    Self {
      nodes: vec![root],
      items: vec![],
    }
  }
  /// Takes an iterator of bounded items and creates an octree from it
  pub fn from(items: impl Iterator<Item=B>) -> Self {
    let items = items.into_iter().collect::<Vec<_>>();
    assert!(!items.is_empty());
    let first_bounds = items[0].bounds();
    let max_bounds = items.iter().skip(1).fold(first_bounds, |acc, n| acc.union(&n.bounds()));
    let mut root = Octant::new(max_bounds);
    root.items.extend(0..items.len());
    Self {
      nodes: vec![root],
      items,
    }
  }
}

impl<D: Float, B: Bounded<D>> Octree<D, B> {
  pub fn push(&mut self, b: B) {
    let bounds = b.bounds();
    let idx = self.items.len();
    self.items.push(b);
    self.nodes[ROOT].push(idx, bounds);
  }
}
