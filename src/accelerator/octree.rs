use super::Accelerator;
use crate::{bounds::Bounds3, interaction::SurfaceInteraction, shapes::Shapes};
use quick_maths::{Ray, Vector};

// DO NOT CHANGE SMALL_SIZE
const SMALL_SIZE: usize = 256;
const MEDIUM_SIZE: usize = 4096;

#[derive(Debug)]
enum NodeStorage {
  // num_items, storage
  Small(u8, Box<Vector<u32, SMALL_SIZE>>),
  Medium(u16, Box<Vector<u32, MEDIUM_SIZE>>),
  // Large(u16, Vector<u32, 65536>),
}

impl NodeStorage {
  fn small() -> Self { NodeStorage::Small(0, Box::new(Vector::of(0))) }
  /// Inserts an item into this node, returning true if the node is full
  fn insert(&mut self, v: u32) -> bool {
    match self {
      NodeStorage::Small(num_items, shapes) => if let Some(ni) = num_items.checked_add(1) {
        shapes[ni as usize] = v;
        *num_items = ni;
        false
      } else {
        shapes[*num_items as usize] = v;
        true
      },
      NodeStorage::Medium(num_items, shapes) => {
        shapes[*num_items as usize] = v;
        *num_items = *num_items + 1;
        *num_items < MEDIUM_SIZE as u16
      },
    }
  }
  /*
  fn is_half_full(&self) -> {
    match self
  }
  */
}

#[derive(Debug)]
struct Node {
  /// Bounds for this node
  bounds: Bounds3,
  /// current number of items in this node
  storage: NodeStorage,
  /// Children of this node
  first_child_idx: u32,
}

impl Node {
  fn new(bounds: Bounds3) -> Self {
    Self {
      bounds,
      storage: NodeStorage::small(),
      first_child_idx: 0,
    }
  }
  fn upgrade_storage(&mut self) {
    let upgrade = match &self.storage {
      NodeStorage::Small(n, v) => NodeStorage::Medium(*n as u16, Box::new(v.zxtend())),
      NodeStorage::Medium(_, _) => panic!("Unimplemented upgrading medium storage size"),
    };
    self.storage = upgrade;
  }
}

#[derive(Debug)]
pub struct Octree {
  shapes: Vec<(Shapes, Bounds3)>,

  nodes: Vec<Node>,
}

impl Octree {
  fn new(bounds: Bounds3, shapes: Vec<(Shapes, Bounds3)>) -> Self {
    let mut out = Self {
      shapes,
      nodes: vec![Node::new(bounds)],
    };
    out.initialize();
    out
  }
  fn initialize(&mut self) {
    assert_eq!(self.nodes.len(), 1, "Should only be run on initializition");
    for (i, (_, bounds)) in self.shapes.iter().enumerate() {
      let node_idx = self.find_smallest_node_containing(bounds, 0);
      let node = &mut self.nodes[node_idx as usize];
      let is_full = node.storage.insert(i as u32);
      if is_full {
        node.upgrade_storage();
        let bounds = node.bounds.octants();
        self.nodes[node_idx as usize].first_child_idx = self.nodes.len() as u32;
        for &bound in bounds.iter() {
          self.nodes.push(Node::new(bound));
        }
      }
    }
  }
  fn find_smallest_node_containing(&self, b: &Bounds3, mut idx: u32) -> u32 {
    loop {
      let curr = &self.nodes[idx as usize];
      debug_assert!(curr.bounds.contains(b));
      let l_octant = curr.bounds.octant_of(&b.min);
      let u_octant = curr.bounds.octant_of(&b.max);
      if l_octant != u_octant { break }
      idx = curr.first_child_idx + (l_octant as u32);
      if idx == 0 { break }
    }
    idx
  }
}

impl Accelerator for Octree {
  fn build(mut i: impl Iterator<Item = Shapes>) -> Self {
    let first_shape = if let Some(shape) = i.next() {
      shape
    } else {
      return Self {
        shapes: vec![],
        nodes: vec![],
      };
    };
    let mut bounds = first_shape.bounds();
    let mut shapes = vec![(first_shape, bounds)];
    shapes.extend(i.map(|s| {
      let bound = s.bounds();
      bounds = bounds.union(&bound);
      (s, bound)
    }));
    // eh whatever this is just for optimization purposes
    shapes.sort_by_cached_key(|(_, b)| b.volume() as u32);
    Octree::new(bounds, shapes)
  }
  fn intersect_ray(&self, _r: &Ray) -> Option<(SurfaceInteraction, &Shapes)> {
    let mut curr = 0;
    let mut best = None;
    todo!();
    best
  }
}
