use crate::vec::Vec3;
use num::{Float, Zero};
use std::{
  collections::{hash_map::Entry, HashMap, HashSet},
  fs::File,
  io::{self, BufRead},
  path::Path,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex<T> {
  pos: Vec3<T>,
  // index into a half_edge list,
  half_edge: usize,
  normal: Vec3<T>,
  // TODO decide whether or not to include a color or not
}

impl<T> Vertex<T> {
  fn connect_he(&mut self, he: &HalfEdge) { self.half_edge = he.id; }
}

/// Iterator over edges adjacent to vertex (vertex-edge iterator).
pub struct VEIter<'a, T> {
  first: &'a HalfEdge,
  curr: &'a HalfEdge,
  mesh: &'a DCEL<T>,
}

impl<'a, T: Float> VEIter<'a, T> {
  fn new(mesh: &'a DCEL<T>, v: &Vertex<T>) -> Self {
    let first = v.he(mesh);
    Self {
      first,
      curr: first,
      mesh,
    }
  }
}

impl<'a, T: Float> Iterator for VEIter<'a, T> {
  type Item = &'a HalfEdge;
  fn next(&mut self) -> Option<Self::Item> {
    let out = self.curr;
    assert_eq!(out.vertex, self.first.vertex);
    let next = self.curr.opp(self.mesh).next(self.mesh);
    if next == self.first {
      return None;
    }
    self.curr = next;
    Some(out)
  }
}

impl<T: Float> Vertex<T> {
  fn new(pos: Vec3<T>) -> Self {
    Self {
      pos,
      half_edge: 0,
      normal: Vec3::zero(),
    }
  }
  pub fn he<'a>(&self, mesh: &'a DCEL<T>) -> &'a HalfEdge { &mesh.half_edges[self.half_edge] }
  pub fn adj_edges<'a>(&self, mesh: &'a DCEL<T>) -> VEIter<'a, T> { VEIter::new(mesh, self) }
  pub fn adj_verts<'a>(&self, mesh: &'a DCEL<T>) -> impl Iterator<Item = &'a Vertex<T>> {
    VEIter::new(mesh, self).map(move |he| he.opp(mesh).vert(mesh))
  }
  pub fn adj_faces<'a>(&self, mesh: &'a DCEL<T>) -> impl Iterator<Item = &'a Face<T>> {
    self.adj_edges(mesh).map(move |he| he.face(mesh))
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct HalfEdge {
  id: usize,
  vertex: usize,
  // index to where the next counterclockwise half edge is, hopefully should be close in the original vector to
  // where this one is counter clockwise(cc)
  cc_next: usize,
  opposite: usize,

  face: usize,
  // TODO decide whether to include a midpoint or not
}

impl HalfEdge {
  pub fn opp<'a, T>(&self, mesh: &'a DCEL<T>) -> &'a HalfEdge { &mesh.half_edges[self.opposite] }
  pub fn next<'a, T>(&self, mesh: &'a DCEL<T>) -> &'a HalfEdge { &mesh.half_edges[self.cc_next] }
  pub fn vert<'a, T>(&self, mesh: &'a DCEL<T>) -> &'a Vertex<T> { &mesh.vertices[self.vertex] }
  pub fn face<'a, T>(&self, mesh: &'a DCEL<T>) -> &'a Face<T> { &mesh.faces[self.face] }
  /// returns the vertices which end this half-edge
  /// In the order of [from, to]
  pub fn adj_verts<'a, T>(&self, mesh: &'a DCEL<T>) -> [&'a Vertex<T>; 2] {
    [self.vert(&mesh), self.opp(&mesh).vert(&mesh)]
  }
  /// returns faces adjacent to this half-edge in the order of this edge and then its opposite
  pub fn adj_faces<'a, T>(&self, mesh: &'a DCEL<T>) -> [&'a Face<T>; 2] {
    [self.face(&mesh), self.opp(&mesh).face(&mesh)]
  }
  /// Returns just the id of adjacent faces
  pub fn adj_face_ids<'a, T>(&self, mesh: &'a DCEL<T>) -> [usize; 2] {
    [self.face, self.opp(&mesh).face]
  }
  /// returns an initial half edge which is partially constructed
  fn build_initial(vertex: usize, face: usize, id: usize) -> Self {
    HalfEdge {
      id,
      vertex,
      face,
      ..Default::default()
    }
  }
  /// Connects two half-edges marking them as opposites
  fn connect_opposite(&mut self, o: &mut Self) {
    self.opposite = o.id;
    o.opposite = self.id;
  }
  /// Builds the counterclock-wise subsequent half-edge
  fn build_next(&mut self, vertex: usize, face: usize, id: usize) -> Self {
    self.cc_next = id;
    HalfEdge {
      id,
      vertex,
      face,
      cc_next: 0,
      opposite: 0,
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Face<T> {
  normal: Option<Vec3<T>>,
  half_edge: usize,
  // TODO decide whether or not to include area or not
  // TODO decide whether to include centroid or not
}

/// Face-Edge iterator
pub struct FEIter<'a, T> {
  /// ID of the first half edge seen
  first: &'a HalfEdge,
  /// Current half edge
  curr: &'a HalfEdge,
  mesh: &'a DCEL<T>,
}

impl<'a, T> Iterator for FEIter<'a, T> {
  type Item = &'a HalfEdge;
  fn next(&mut self) -> Option<Self::Item> {
    let out = self.curr;
    let next = self.curr.next(self.mesh);
    if next == self.first {
      return None;
    }
    self.curr = next;
    Some(out)
  }
}

impl<T> Face<T> {
  fn from_he(half_edge: &HalfEdge) -> Self {
    Self {
      half_edge: half_edge.id,
      normal: None,
    }
  }
  fn he<'a>(&self, mesh: &'a DCEL<T>) -> &'a HalfEdge { &mesh.half_edges[self.half_edge] }
  /// Returns edges adjacent to this face
  pub fn adj_edges<'a>(&self, mesh: &'a DCEL<T>) -> FEIter<'a, T> {
    let first = self.he(mesh);
    FEIter {
      first,
      curr: first,
      mesh,
    }
  }
  /// Returns vertices adjacent to this face
  pub fn adj_verts<'a>(&self, mesh: &'a DCEL<T>) -> impl Iterator<Item = &'a Vertex<T>> {
    self.adj_edges(mesh).map(move |he| he.vert(mesh))
  }
  pub fn adj_faces<'a>(&self, mesh: &'a DCEL<T>) -> impl Iterator<Item = &'a Face<T>> {
    self.adj_edges(mesh).map(move |he| he.face(mesh))
  }
}

/// A topographic list of graph elements.
/// They are not in any particular order, but in theory adjacent elements
/// should be next to each other for cache efficiency.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct DoublyConnectedEdgeList<T = f32> {
  vertices: Vec<Vertex<T>>,
  faces: Vec<Face<T>>,
  half_edges: Vec<HalfEdge>,
}

// dcel is a Doubly Connected Edge List
pub type DCEL<T> = DoublyConnectedEdgeList<T>;

impl<T: Float> DoublyConnectedEdgeList<T> {
  /// Constructs a non-empty DoublyConnectedEdgeList from a set of vertices and their faces
  pub fn new(vertices: Vec<Vec3<T>>, faces: &[Vec<usize>]) -> Self {
    let mut vertices = vertices.into_iter().map(Vertex::new).collect::<Vec<_>>();
    let mut hes = vec![];
    let mut finished_faces: Vec<Face<T>> = vec![];
    assert!(!faces.is_empty());
    assert!(!vertices.is_empty());
    let mut count = 0;
    let mut new_id = || -> usize {
      let curr: usize = count;
      count += 1;
      curr
    };
    let mut he: HashMap<(usize, usize), HalfEdge> = HashMap::new();
    let mut complete = |from: usize, to: usize, mut curr: HalfEdge| match he.entry((to, from)) {
      Entry::Occupied(o) => {
        let mut opp = o.remove();
        curr.connect_opposite(&mut opp);
        vertices[from].connect_he(&curr);
        hes.push(opp);
        hes.push(curr);
      },
      Entry::Vacant(_) => assert!(he.insert((from, to), curr).is_none()),
    };
    for (f, verts) in faces.iter().enumerate() {
      assert!(verts.len() >= 3);
      let mut curr_vert = verts[0];
      let mut curr = HalfEdge::build_initial(curr_vert, f, new_id());
      let first_id = curr.id;
      finished_faces.push(Face::from_he(&curr));
      for &v in &verts[1..] {
        let next = curr.build_next(v, f, new_id());
        complete(curr_vert, v, curr);
        curr_vert = v;
        curr = next;
      }
      // need to link last created one to the first half_edge
      curr.cc_next = first_id;
      assert_eq!(curr_vert, *verts.last().unwrap());
      assert_ne!(curr_vert, verts[0]);
      complete(curr_vert, verts[0], curr);
    }
    // have to sort here because the halfedges are processed in random order
    hes.sort_unstable_by_key(|he| he.id);
    // assert!(he.is_empty(), "Still had remaining half-edges {:?}", he);
    Self {
      vertices,
      faces: finished_faces,
      half_edges: hes,
    }
  }
  /// Creates a DoublyConnectedEdgeList from an obj file
  pub fn from_obj<P: AsRef<Path>>(p: P) -> io::Result<Self> {
    let f = File::open(p)?;
    let buf = io::BufReader::new(f);
    let mut positions = vec![];
    let mut faces = vec![];
    for line in buf.lines() {
      let line = line?;
      let parts = line.split_whitespace().collect::<Vec<_>>();
      match parts.as_slice() {
        [] | ["#", ..] => (),
        ["g", ..] => {},
        ["v", x, y, z] => {
          let pos = Vec3::<T>::from_str_radix((x, y, z), 10).unwrap_or_else(|_| todo!());
          positions.push(pos);
        },
        ["vn", x, y, z] => {
          let _norm = Vec3::<T>::from_str_radix((x, y, z), 10).unwrap_or_else(|_| todo!());
        },
        ["vt", _u, _v, _w] => todo!(),
        ["p", _vs @ ..] => todo!(),
        ["f", fs @ ..] => {
          let uniq = fs.iter().collect::<HashSet<_>>();
          if uniq.len() < 3 {
            // degenerate faces cannot be handled
            continue;
          }
          faces.push(
            fs.iter()
              .map(|v| {
                let mut items = v.split('/');
                let vertex = items
                  .next()
                  .expect("Expected at least a vertex element")
                  .parse::<usize>()
                  .expect("Could not parse vertex index")
                  - 1;
                let _texture = items
                  .next()
                  .and_then(|it| it.parse::<usize>().ok())
                  .map(|it| it - 1);
                let _normal = items
                  .next()
                  .and_then(|it| it.parse::<usize>().ok())
                  .map(|it| it - 1);
                vertex
              })
              .collect(),
          );
        },
        ["s", "off"] | ["s", "0"] => (),
        v => panic!("Not yet implemented {:?}", v),
      };
    }
    Ok(Self::new(positions, &faces))
  }
  #[cfg(test)]
  /// validates that the mesh correctly satisfies the invariants of a DCEL
  fn validate(&self) {
    // check that faces match up
    let mismatched_faces = self
      .faces
      .iter()
      .enumerate()
      .filter_map(|(i, f)| {
        let supposed_face = self.half_edges[f.half_edge].face;
        Some((i, supposed_face)).filter(|(i, f)| i != f)
      })
      .collect::<Vec<_>>();
    assert!(mismatched_faces.is_empty());
    let mismatched_runs = self
      .faces
      .iter()
      .enumerate()
      .filter_map(|(i, f)| {
        let init_he = f.he(self);
        let mut curr = init_he;
        loop {
          if curr.face != i {
            return Some(curr);
          } else if curr == init_he {
            break;
          }
          curr = curr.next(self);
        }
        None
      })
      .collect::<Vec<_>>();
    assert!(mismatched_runs.is_empty());
    let mismatched_opps = self
      .half_edges
      .iter()
      .filter_map(|he| Some(he).filter(|he| &he.opp(self).opp(self) != he))
      .collect::<Vec<_>>();
    assert!(mismatched_opps.is_empty());
    let mismatched_verts = self
      .vertices
      .iter()
      .filter_map(|v| Some(v).filter(|v| &v.he(self).vert(self) != v))
      .collect::<Vec<_>>();
    assert!(mismatched_verts.is_empty());
  }
  /// Creates a new vertex and returns its id
  fn new_vert(&mut self, pos: Vec3<T>) -> usize {
    let id = self.vertices.len();
    self.vertices.push(Vertex::new(pos));
    id
  }
  // Converts each face of self into a triangle
  pub fn triangulate(&mut self) {
    todo!();
  }
  fn update_half_edge(&mut self, he: HalfEdge) { self.half_edges[he.id] = he; }
  /// splits a half edge by inserting a vertex a distance of factor between the two endpoints
  /// Returns the new vertex created and discards the half-edge originally passed into the
  pub fn split_half_edge(&mut self, mut he: HalfEdge, factor: T) -> usize {
    let factor = factor.min(T::one()).max(T::zero());
    let next_id = he.next(self).id;
    let mut opp = *he.opp(self);
    let opp_next_id = opp.next(self).id;
    // goes from v1 to v2
    let [v1, v2] = he.adj_verts(&self);
    let [f1, f2] = he.adj_face_ids(&self);
    let (p1, p2) = (v1.pos, v2.pos);
    let new_vert = self.new_vert(Vec3::lerp(factor, p1, p2));
    // new_vert -> v2
    let mut new_he1 = HalfEdge::build_initial(new_vert, f1, self.half_edges.len());
    // new_vert -> v1
    let mut new_he2 = HalfEdge::build_initial(new_vert, f2, self.half_edges.len() + 1);
    he.cc_next = new_he1.id;
    new_he1.cc_next = next_id;
    he.connect_opposite(&mut new_he2);

    opp.cc_next = new_he2.id;
    new_he2.cc_next = opp_next_id;
    opp.connect_opposite(&mut new_he1);
    self.half_edges.push(new_he1);
    self.half_edges.push(new_he2);
    self.update_half_edge(he);
    self.update_half_edge(opp);

    new_vert
  }
}

#[test]
fn test_dcel() {
  let p = Path::new(file!())
    .parent()
    .unwrap()
    .join("sample_files")
    .join("cube.obj");
  let mesh = match DCEL::<f32>::from_obj(p) {
    Ok(m) => m,
    Err(e) => panic!("failed to parse mesh {}", e),
  };
  mesh.validate();
  let f0 = &mesh.faces[0];
  let ok = f0.adj_edges(&mesh).all(|he| he.face(&mesh) == f0);
  assert!(ok);
  let v0 = &mesh.vertices[0];
  let ok = v0.adj_edges(&mesh).all(|he| he.vert(&mesh) == v0);
  assert!(ok);
}
