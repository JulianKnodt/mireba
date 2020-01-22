use crate::vec::{Ray, Vec3};
use num::Float;

/// Axis Aligned bounding box
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bounds<Dim>([Vec3<Dim>; 2]);
impl<D: Float> Bounds<D> {
  /// Returns the minimum of this bounding box
  pub fn min(&self) -> &Vec3<D> { &self.0[0] }
  pub fn max(&self) -> &Vec3<D> { &self.0[1] }
  /// Creates a new bound assuming that the passed vec3 are appropriately min-max
  pub(crate) fn new(a: [Vec3<D>; 2]) -> Self {
    assert!(a[0].0 <= a[1].0);
    assert!(a[0].1 <= a[1].1);
    assert!(a[0].2 <= a[1].2);
    Bounds(a)
  }
  pub fn valid(a: [Vec3<D>; 2]) -> Self { Bounds([a[0].min_parts(&a[1]), a[0].max_parts(&a[1])]) }
  /// Returns whether this bounding box contains the other.
  /// If they have the same coordinates for one of the sides it will still return true
  pub fn contains(&self, o: &Self) -> bool { self.max() >= o.max() && self.min() <= o.min() }

  /// Returns whether edges of the other bounding box are fully contained in this one.
  pub fn stricly_contains(&self, o: &Self) -> bool { self.max() > o.max() && self.min() < o.min() }
  pub fn union(&self, o: &Self) -> Self {
    Self::new([self.min().min_parts(o.min()), self.max().max_parts(o.max())])
  }
  /// Returns the volume inside this bounding box
  pub fn volume(&self) -> D {
    let &Vec3(lx, ly, lz) = self.min();
    let &Vec3(hx, hy, hz) = self.max();
    (hx - lx) * (hy - ly) * (hz - lz)
  }
  /// Returns whether this bounding box intersects this ray
  pub fn intersects_ray(&self, r: &Ray<D>) -> bool {
    let &Vec3(lx, ly, lz) = self.min();
    let &Vec3(hx, hy, hz) = self.max();
    let &Vec3(px, py, pz) = &r.pos;
    let &Vec3(dx, dy, dz) = &r.dir;

    let t = |coord, pos, dir| (coord - pos) / dir;
    let (thx, tlx) = (t(hx, px, dx), t(lx, px, dx));
    let (thy, tly) = (t(hy, py, dy), t(ly, py, dy));
    let (thz, tlz) = (t(hz, pz, dz), t(lz, pz, dz));
    let t_max = D::infinity()
      .min(thx.max(tlx))
      .min(thy.max(tly))
      .min(thz.max(tlz));
    let t_min = D::neg_infinity()
      .max(thx.min(tlx))
      .max(thy.min(tly))
      .max(thz.min(tlz));

    t_max > t_min.max(D::zero())
  }
  pub fn intersects_box(&self, o: &Self) -> bool {
    let &Vec3(lx, ly, lz) = self.min();
    let &Vec3(hx, hy, hz) = self.max();
    let &Vec3(olx, oly, olz) = o.min();
    let &Vec3(ohx, ohy, ohz) = o.max();
    (olx > lx && olx < hx)
      || (ohx < hx && ohx > lx) && (oly > ly && oly < hy)
      || (ohy < hy && ohy > ly) && (olz > lz && olz < hz)
      || (ohz < hz && ohz > lz)
  }
}

fn octants<D: Float>(bounds: Bounds<D>) -> [Bounds<D>; 8] {
  let center = (*bounds.min() + *bounds.max()) / D::from(2.0).unwrap();
  let Vec3(cx, cy, cz) = center;
  let &Vec3(lx, ly, lz) = bounds.min();
  let &Vec3(hx, hy, hz) = bounds.max();
  let xyz = Bounds::new([center, *bounds.max()]);
  let nxyz = Bounds::new([Vec3(lx, cy, cz), Vec3(cx, hy, hz)]);
  let xnyz = Bounds::new([Vec3(cx, ly, cz), Vec3(hx, cy, hz)]);
  let xynz = Bounds::new([Vec3(cx, cy, lz), Vec3(hx, hy, cz)]);

  let nxnyz = Bounds::new([Vec3(lx, ly, cz), Vec3(cx, cy, hz)]);
  let xnynz = Bounds::new([Vec3(cx, ly, lz), Vec3(hx, cy, cz)]);
  let nxynz = Bounds::new([Vec3(lx, cy, lz), Vec3(cx, hy, cz)]);
  let nxnynz = Bounds::new([*bounds.min(), center]);
  // order shouldn't matter here
  [xyz, nxyz, xnyz, xynz, nxnyz, xnynz, nxynz, nxnynz]
}

pub trait Bounded<D> {
  /// returns the bounds for this object
  /// Should be relatively cheap so it can be called multiple times
  fn bounds(&self) -> Bounds<D>;
}

impl<D: Clone> Bounded<D> for Bounds<D> {
  fn bounds(&self) -> Bounds<D> { self.clone() }
}

impl<D: Clone> Bounded<D> for Vec3<D> {
  fn bounds(&self) -> Bounds<D> { Bounds([self.clone(), self.clone()]) }
}

impl<D> Bounded<D> for Box<dyn Bounded<D> + '_> {
  fn bounds(&self) -> Bounds<D> { self.as_ref().bounds() }
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
  root: OctreeInner<D>,
  /// Owning storage for this octree
  backing_store: Vec<T>,
}

impl<K, T> Octree<K, T> {
  pub fn len(&self) -> usize { self.root.len() }
  /// Iterates over the items in this octree in no particular order.
  pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
    self.root.iter().map(move |v| &self.backing_store[v])
  }
}

impl<D: Float, B: Bounded<D>> Octree<D, B> {
  pub fn new(bounds: Bounds<D>, ps: PartitionStrategy) -> Self {
    Octree {
      root: OctreeInner::new(&bounds, ps),
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
      .filter(move |v| v.bounds().intersects_ray(&r))
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

impl<D: Float, B: Bounded<D>> std::iter::FromIterator<B> for Octree<D, B> {
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
      root: OctreeInner::new(&complete_bounds, Default::default()),
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
      root: OctreeInner::new(&complete_bounds, Default::default()),
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
enum OctreeVariant<D> {
  /// Holds the elements themselves
  Leaf(Vec<usize>, PartitionStrategy),
  /// Holds smaller components which themselves should contain leaves
  // TODO determine whether partitioned octree should contain items
  Partitioned([Box<OctreeInner<D>>; 8]),
}

impl<D> OctreeVariant<D> {
  pub fn len(&self) -> usize {
    match self {
      OctreeVariant::Leaf(v, _) => v.len(),
      OctreeVariant::Partitioned(sub) => sub.iter().map(|s| s.len()).sum(),
    }
  }
}

/// OctreeInner is a node inside of the octree, that contains references to the backing store
#[derive(Debug, Clone, PartialEq)]
struct OctreeInner<D> {
  /// The bounds on this inner octree node
  bounds: Bounds<D>,
  /// What kind of octree is this inner node?
  var: OctreeVariant<D>,
}

impl<D> OctreeInner<D> {
  pub fn len(&self) -> usize { self.var.len() }
  /// Iterates over the items in this octree in no particular order.
  pub fn iter(&self) -> Box<dyn Iterator<Item = usize> + '_> {
    match &self.var {
      OctreeVariant::Leaf(items, _) => Box::new(items.iter().copied()),
      OctreeVariant::Partitioned(octs) => Box::new(octs.iter().flat_map(|p| p.iter())),
    }
  }
}

impl<D: Float> OctreeInner<D> {
  pub fn new(bounds: &Bounds<D>, ps: PartitionStrategy) -> Self {
    Self {
      var: OctreeVariant::Leaf(vec![], ps),
      bounds: *bounds,
    }
  }
  fn extend<I: IntoIterator<Item = usize>, B: Bounded<D>>(
    &mut self,
    iter: I,
    backing_store: &[B],
  ) {
    for v in iter {
      let bounds = backing_store[v].bounds();
      self.add_unchecked(v, &bounds);
    }
    self.partition_recursive(&backing_store);
  }
  fn partition_recursive<B: Bounded<D>>(&mut self, backing_store: &[B]) {
    match self.var {
      OctreeVariant::Leaf(_, _) =>
        if self.should_partition() {
          self.partition(&backing_store)
        },
      OctreeVariant::Partitioned(ref mut children) =>
        return children
          .iter_mut()
          .for_each(|child| child.partition_recursive(&backing_store)),
    };
  }
  /// Partitions this octree into 8 iff it's a leaf
  fn partition<B: Bounded<D>>(&mut self, backing_store: &[B]) {
    let octants = octants(self.bounds);
    // sanity check TODO remove this once I actually test this is true
    debug_assert!(octants.iter().all(|oct| self.bounds.contains(oct)));
    let ps = match self.var {
      // Should this check happen earlier? It really should never occur
      OctreeVariant::Partitioned(_) => panic!("Unexpected partition of partitioned octree"),
      OctreeVariant::Leaf(_, ps) => ps,
    };
    // is cloning the partition strategy a good approach? Probably not but it's like two unsized
    // integers but it doesn't matter
    let children = [
      Box::new(OctreeInner::new(&octants[0], ps)),
      Box::new(OctreeInner::new(&octants[1], ps)),
      Box::new(OctreeInner::new(&octants[2], ps)),
      Box::new(OctreeInner::new(&octants[3], ps)),
      Box::new(OctreeInner::new(&octants[4], ps)),
      Box::new(OctreeInner::new(&octants[5], ps)),
      Box::new(OctreeInner::new(&octants[6], ps)),
      Box::new(OctreeInner::new(&octants[7], ps)),
    ];
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
  fn add<B: Bounded<D>>(
    &mut self,
    idx: usize,
    bounds: &Bounds<D>,
    backing_store: &[B],
  ) -> bool {
    if !self.add_unchecked(idx, bounds) {
      return false;
    }
    match self.var {
      OctreeVariant::Partitioned(ref mut children) => children
        .iter_mut()
        .any(|child| child.add(idx, &bounds, &backing_store)),
      OctreeVariant::Leaf(ref mut indeces, _) => {
        indeces.push(idx);
        if self.should_partition() {
          self.partition(&backing_store);
        }
        true
      },
    }
  }
  // adds an item to this octree unchecked without partitioning if conditions are condition
  fn add_unchecked(&mut self, idx: usize, bounds: &Bounds<D>) -> bool {
    if !self.bounds.contains(bounds) {
      return false;
    }
    match &mut self.var {
      OctreeVariant::Partitioned(ref mut children) => children
        .iter_mut()
        .any(|child| child.add_unchecked(idx, &bounds)),
      OctreeVariant::Leaf(items, _) => {
        items.push(idx);
        true
      },
    }
  }
  fn intersecting_elems(&self, r: Ray<D>) -> Box<dyn Iterator<Item = usize> + '_> {
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
  use super::{Bounds, Octree};
  use crate::vec::Vec3;
  fn rand_bounds() -> Bounds<f32> {
    Bounds::valid([
      Vec3(rand::random(), rand::random(), rand::random()),
      Vec3(rand::random(), rand::random(), rand::random()),
    ])
  }
  #[test]
  fn build_octree() {
    // any higher is too slow for debug releases
    let n = 10000;
    let oct = (0..n).map(|_| rand_bounds()).collect::<Octree<_, _>>();
    assert_eq!(oct.len(), n);
  }
}
