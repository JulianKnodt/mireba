use crate::{
  bounds::{Bounded, Bounds},
  vec::{Ray, Vec3},
};
use num::Float;

fn octants<D: Float>(bounds: Bounds<D>) -> [Bounds<D>; 8] {
  let center = (*bounds.min() + *bounds.max()) / D::from(2.0).unwrap();
  let Vec3(cx, cy, cz) = center;
  let &Vec3(lx, ly, lz) = bounds.min();
  let &Vec3(hx, hy, hz) = bounds.max();
  [
    Bounds::new([center, *bounds.max()]),
    Bounds::new([Vec3(lx, cy, cz), Vec3(cx, hy, hz)]),
    Bounds::new([Vec3(cx, ly, cz), Vec3(hx, cy, hz)]),
    Bounds::new([Vec3(cx, cy, lz), Vec3(hx, hy, cz)]),
    Bounds::new([Vec3(cx, ly, lz), Vec3(hx, cy, cz)]),
    Bounds::new([Vec3(lx, cy, lz), Vec3(cx, hy, cz)]),
    Bounds::new([Vec3(lx, ly, cz), Vec3(cx, cy, hz)]),
    Bounds::new([*bounds.min(), center]),
  ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PartitionStrategy {
  /// maximum number of values allowed in a leaf node
  max_values: usize,
  // /// Minimum allowed volume
  // min_volume: D,
}

impl Default for PartitionStrategy {
  fn default() -> Self { Self { max_values: 256 } }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Octree<D, T> {
  /// The roof of this octree, that wraps all inner octree nodes.
  pub root: OctreeInner<D>,
  /// Owning storage for this octree
  backing_store: Vec<T>,
}

impl<K, T> Octree<K, T> {
  pub fn len(&self) -> usize { self.root.len() }
  pub fn max_depth(&self) -> usize { self.root.max_depth() }
  pub fn is_empty(&self) -> bool { self.root.len() == 0 }
  /// Iterates over the items in this octree in no particular order.
  pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
    self.root.iter().map(move |v| &self.backing_store[v])
  }
}

impl<D: Float, B: Bounded<D>> Octree<D, B> {
  pub fn new(bounds: Bounds<D>, ps: PartitionStrategy) -> Self {
    Octree {
      root: OctreeInner::new(bounds, ps),
      backing_store: vec![],
    }
  }
  /// adds an item to the given octree and returns whether or not it was successfully it was in
  /// bounds of the octree.
  pub fn push(&mut self, item: B) -> bool {
    let bounds = item.bounds();
    let idx = self.backing_store.len();
    if bounds.contains(&self.root.bounds) {
      // need to reconstruct whole octree
      todo!()
    }
    self.backing_store.push(item);
    self.root.add(idx, &bounds, &self.backing_store)
  }
  pub(crate) fn push_unchecked(&mut self, item: B) -> bool {
    let bounds = item.bounds();
    let idx = self.backing_store.len();
    self.backing_store.push(item);
    self.root.add_unchecked(idx, &bounds)
  }
  pub fn partition_recursive(&mut self) { self.root.partition_recursive(&self.backing_store) }
  // TODO determine how to convert this ray into a reference
  pub fn intersecting_elements(&self, r: Ray<D>) -> impl Iterator<Item = &B> + '_ {
    self
      .root
      .intersecting_elems(r)
      .map(move |i| &self.backing_store[i])
  }
  /// TODO delete this temporary method that partitions the tree once
  pub fn partition(&mut self) { self.root.partition(&self.backing_store); }
  /// Prepares this octree for rendering
  /// by more aggresively partition leaves.
  pub fn prepare(&mut self) {
    self.root.partition_aggressive(&self.backing_store);
    self.backing_store.shrink_to_fit();
  }
}

impl<D: Float, I: Bounded<D>> Extend<I> for Octree<D, I> {
  fn extend<T: IntoIterator<Item = I>>(&mut self, iter: T) {
    for v in iter {
      self.push_unchecked(v);
    }
    self.partition_recursive();
  }
}

impl<B: Bounded<f32>> std::iter::FromIterator<B> for Octree<f32, B> {
  fn from_iter<I: IntoIterator<Item = B>>(iter: I) -> Self {
    let backing_store = iter.into_iter().collect::<Vec<_>>();
    assert!(
      !backing_store.is_empty(),
      "Cannot construct Octree from empty iterator"
    );
    let initial_bounds = backing_store[0].bounds();
    let complete_bounds = backing_store
      .iter()
      .skip(1)
      .fold(initial_bounds, |acc, next| acc.union(&next.bounds()));
    let mut out = Self {
      root: OctreeInner::new(complete_bounds, Default::default()),
      backing_store,
    };
    for i in 0..out.backing_store.len() {
      out.root.add_unchecked(i, &out.backing_store[i].bounds());
    }
    out
  }
}

impl<'m, D: Float, B: Bounded<D>> From<Vec<B>> for Octree<D, B> {
  fn from(backing_store: Vec<B>) -> Self {
    assert!(
      !backing_store.is_empty(),
      "Cannot construct Octree from empty iterator"
    );
    let initial_bounds = backing_store[0].bounds();
    let complete_bounds = backing_store
      .iter()
      .skip(1)
      .fold(initial_bounds, |acc, next| acc.union(&next.bounds()));
    let mut out = Self {
      root: OctreeInner::new(complete_bounds, Default::default()),
      backing_store,
    };
    for i in 0..out.backing_store.len() {
      out.root.add_unchecked(i, &out.backing_store[i].bounds());
    }
    out
  }
}

// Octree Variant implementation

/// Octree variant specifies whether an octree node is a leaf or a partitioned node
#[derive(Debug, Clone, PartialEq)]
pub enum OctreeVariant<D> {
  /// Holds the elements themselves
  Leaf(Vec<usize>, PartitionStrategy),
  /// Holds smaller components which themselves should contain leaves
  // TODO determine whether partitioned octree should contain items
  Partitioned(Box<[OctreeInner<D>; 8]>),
}

impl<D> OctreeVariant<D> {
  pub fn max_depth(&self) -> usize {
    match self {
      OctreeVariant::Leaf(_, _) => 1,
      OctreeVariant::Partitioned(children) =>
        children
          .iter()
          .map(|child| child.max_depth())
          .max()
          .unwrap()
          + 1,
    }
  }
  pub fn len(&self) -> usize {
    match self {
      OctreeVariant::Leaf(v, _) => v.len(),
      OctreeVariant::Partitioned(sub) => sub.iter().map(|s| s.len()).sum(),
    }
  }
}

/// OctreeInner is a node inside of the octree, that contains references to the backing store
#[derive(Debug, Clone, PartialEq)]
pub struct OctreeInner<D> {
  /// The bounds on this inner octree node
  pub bounds: Bounds<D>,
  /// What kind of octree is this inner node?
  pub var: OctreeVariant<D>,
}

impl<D> OctreeInner<D> {
  pub fn len(&self) -> usize { self.var.len() }
  pub fn max_depth(&self) -> usize { self.var.max_depth() }
  pub fn bounds(&self) -> &Bounds<D> { &self.bounds }
  /// Iterates over the items in this octree in no particular order.
  pub fn iter(&self) -> Box<dyn Iterator<Item = usize> + '_> {
    match &self.var {
      OctreeVariant::Leaf(items, _) => Box::new(items.iter().copied()),
      OctreeVariant::Partitioned(octs) => Box::new(octs.iter().flat_map(|p| p.iter())),
    }
  }
}

impl<D: Float> OctreeInner<D> {
  pub fn new(bounds: Bounds<D>, ps: PartitionStrategy) -> Self {
    Self {
      var: OctreeVariant::Leaf(vec![], ps),
      bounds,
    }
  }
  fn extend<I: IntoIterator<Item = usize>, B: Bounded<D>>(&mut self, iter: I, items: &[B]) {
    for v in iter {
      let bounds = items[v].bounds();
      assert!(self.add_unchecked(v, &bounds));
    }
    self.partition_recursive(&items);
  }
  fn partition_recursive<B: Bounded<D>>(&mut self, backing_store: &[B]) {
    match self.var {
      OctreeVariant::Leaf(_, _) =>
        if self.should_partition() {
          self.partition(&backing_store)
        },
      OctreeVariant::Partitioned(ref mut children) => children
        .iter_mut()
        .for_each(|child| child.partition_recursive(&backing_store)),
    };
  }
  /// aggressively partition this node
  fn partition_aggressive<B: Bounded<D>>(&mut self, backing_store: &[B]) {
    match self.var {
      OctreeVariant::Partitioned(ref mut children) =>
        return children
          .iter_mut()
          .for_each(|child| child.partition_aggressive(&backing_store)),
      OctreeVariant::Leaf(ref idxs, _) => {
        // volume of one partitioned node
        let vol = self.bounds.volume() / D::from(8.0).unwrap();
        let should_partition = idxs
          .iter()
          .any(|&i| backing_store[i].bounds().volume() < vol);
        if should_partition {
          self.partition(&backing_store);
        } else {
          return;
        }
      },
    };
    self.partition_aggressive(&backing_store);
  }
  /// Partitions this octree into 8 iff it's a leaf
  pub fn partition<B: Bounded<D>>(&mut self, backing_store: &[B]) {
    let octants = octants(self.bounds);
    // sanity check TODO remove this once I actually test this is true
    assert!(octants.iter().all(|oct| self.bounds.contains(oct)));
    let ps = match self.var {
      OctreeVariant::Partitioned(_) => panic!("Unexpected partition of partitioned octree"),
      OctreeVariant::Leaf(_, ps) => ps,
    };
    // is cloning the partition strategy a good approach? It's only two unsigned
    // integers so it likely doesn't matter
    let children = Box::new([
      OctreeInner::new(octants[0], ps),
      OctreeInner::new(octants[1], ps),
      OctreeInner::new(octants[2], ps),
      OctreeInner::new(octants[3], ps),
      OctreeInner::new(octants[4], ps),
      OctreeInner::new(octants[5], ps),
      OctreeInner::new(octants[6], ps),
      OctreeInner::new(octants[7], ps),
    ]);
    let out = std::mem::replace(&mut self.var, OctreeVariant::Partitioned(children));
    let mut items = if let OctreeVariant::Leaf(items, _) = out {
      items
    } else {
      unreachable!()
    };
    self.extend(items.drain(..), &backing_store);
  }
  /// returns whether the current octree should partition
  /// based on a given partition strategy
  fn should_partition(&self) -> bool {
    match &self.var {
      OctreeVariant::Partitioned(_) => false,
      OctreeVariant::Leaf(items, ps) => items.len() > ps.max_values,
    }
  }
  fn add<B: Bounded<D>>(&mut self, idx: usize, bounds: &Bounds<D>, backing_store: &[B]) -> bool {
    if !self.add_unchecked(idx, bounds) {
      return false;
    }
    match self.var {
      OctreeVariant::Partitioned(ref mut children) => children
        .iter_mut()
        .map(|child| child.add(idx, &bounds, &backing_store))
        .any(|b| b),
      OctreeVariant::Leaf(ref mut indeces, _) => {
        indeces.push(idx);
        if self.should_partition() {
          self.partition(&backing_store);
        }
        true
      },
    }
  }
  /// adds an item to this octree unchecked without partitioning if conditions are condition
  fn add_unchecked(&mut self, idx: usize, bounds: &Bounds<D>) -> bool {
    if !self.bounds.intersects_box(bounds) {
      return false;
    }
    match &mut self.var {
      OctreeVariant::Partitioned(ref mut children) => children
        .iter_mut()
        .map(|child| child.add_unchecked(idx, &bounds))
        .any(|b| b),
      OctreeVariant::Leaf(items, _) => {
        items.push(idx);
        true
      },
    }
  }
  // TODO convert this dynamic iterator into one of a couple kinds? Is that possible here?
  fn intersecting_elems(&self, r: Ray<D>) -> Box<dyn Iterator<Item = usize> + '_> {
    // This bounds check is incorrectly returning objects
    if !self.bounds.intersects_ray(&r) {
      return Box::new(std::iter::empty());
    };
    match self.var {
      OctreeVariant::Partitioned(ref children) => Box::new(
        children
          .iter()
          .flat_map(move |child| child.intersecting_elems(r)),
      ),
      OctreeVariant::Leaf(ref items, _) => Box::new(items.iter().copied()),
    }
  }
}

// octree tests
#[cfg(test)]
mod test {
  use super::{octants, Bounds, Octree};
  use crate::vec::Vec3;
  fn rand_bounds() -> Bounds<f32> {
    Bounds::valid([
      Vec3(rand::random(), rand::random(), rand::random()),
      Vec3(rand::random(), rand::random(), rand::random()),
    ])
  }
  fn unit_bounds() -> Bounds<f32> { Bounds::valid([Vec3::from(0.0), Vec3::from(1.0)]) }
  #[test]
  fn build_octree() {
    // any higher is too slow for debug releases
    let n = 10000;
    let oct = (0..n).map(|_| rand_bounds()).collect::<Octree<_, _>>();
    assert_eq!(oct.len(), n);
  }

  #[test]
  fn correct_octants() {
    let b = unit_bounds();
    let octs = octants(b);
    assert!((b.volume() - octs.iter().map(|v| v.volume()).sum::<f32>()).abs() < 0.000001);
    octs.iter().enumerate().for_each(|(i, oct1)| {
      octs
        .iter()
        .enumerate()
        .filter(|(j, _)| *j != i)
        .for_each(|(_, oct2)| {
          assert_ne!(oct1, oct2);
          assert!(!oct1.strictly_contains(&oct2));
          assert!(!oct1.intersects_box(&oct2), "{:?} {:?}", oct1, oct2);
          assert_eq!(oct1.volume(), oct2.volume());
        })
    });
    assert!(octs.iter().all(|oct| b.contains(oct)));
    println!("{:?}", octs);
  }
}
