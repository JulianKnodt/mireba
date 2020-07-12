use super::Accelerator;
use crate::{
  bounds::{Bounded, Bounds3},
  interaction::SurfaceInteraction,
  shapes::Shapes,
};
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
  /// Creates a small node storage instance
  fn small() -> Self { NodeStorage::Small(0, Box::new(Vector::of(0))) }

  /// Inserts an item into this node, returning true if the node is full
  fn insert(&mut self, v: u32) -> bool {
    match self {
      NodeStorage::Small(num_items, shapes) =>
        if let Some(ni) = num_items.checked_add(1) {
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
  fn items(&self) -> &[u32] {
    match self {
      NodeStorage::Small(l, data) => &data.0[..*l as usize],
      NodeStorage::Medium(l, data) => &data.0[..*l as usize],
    }
  }
}

#[derive(Debug)]
struct Node {
  /// Bounds for this node
  bounds: Bounds3,
  /// current idxs of items in this node
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
  const fn has_children(&self) -> bool { self.first_child_idx != 0 }
  /*
  fn children_indeces(&self) -> impl Iterator<Item = u32> {
    self.first_child_idx..self.first_child_idx + 8
  }
  */
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
    out.init();
    out
  }
  fn init(&mut self) {
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
      if l_octant != u_octant {
        break;
      }
      let next_idx = curr.first_child_idx + (l_octant.inner() as u32);
      if next_idx == 0 {
        break;
      }
      idx = next_idx
    }
    idx
  }
  pub fn total_bounds(&self) -> &Bounds3 { &self.nodes[0].bounds }
  /// Checks all items for a given node and ray
  fn node_intersect_ray(&self, node_idx: u32, r: &Ray) -> Option<(SurfaceInteraction, &Shapes)> {
    let node = &self.nodes[node_idx as usize];
    node.storage.items().iter().fold(None, |prev, &i| {
      let shape = &self.shapes[i as usize].0;
      if let Some(si) = shape.intersect_ray(r) {
        Some(if let Some((prev_si, prev_shape)) = prev {
          if prev_si.it.t < si.it.t {
            (prev_si, prev_shape)
          } else {
            (si, shape)
          }
        } else {
          (si, shape)
        })
      } else {
        prev
      }
    })
  }
  fn naive_intersect(&self, node_idx: u32, ray: &Ray) -> Option<(SurfaceInteraction, &Shapes)> {
    let own_intersection = self.node_intersect_ray(node_idx, ray);
    let node = &self.nodes[node_idx as usize];
    if !node.has_children() {
      return own_intersection;
    }
    let dir = ray.dir.is_sign_positive();
    let curr_position = node.bounds.octant_of(&ray.pos);
    curr_position
      .in_dir(&dir)
      .map(|oo| node.first_child_idx + oo.inner() as u32)
      .map(|node_idx| self.naive_intersect(node_idx, ray))
      .chain(std::iter::once(own_intersection))
      .filter_map(|v| v)
      .min_by(|a, b| a.0.it.t.partial_cmp(&b.0.it.t).unwrap())
  }
}

impl Bounded for Octree {
  fn bounds(&self) -> Bounds3 { self.nodes[0].bounds }
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
  fn intersect_ray(&self, r: &Ray) -> Option<(SurfaceInteraction, &Shapes)> {
    self.naive_intersect(0, r)
  }
}
