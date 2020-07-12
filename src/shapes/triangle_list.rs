use super::{triangle::Triangle, Shape};
use crate::{
  bounds::{Bounded, Bounds3},
  interaction::SurfaceInteraction,
  utils::triangulate,
};
use quick_maths::{Ray, Vec3, Vector};
use std::{fs::File, io, io::BufRead, path::Path, str::FromStr};

/// A group of faces
#[derive(Debug, Default)]
pub struct FaceGroup {
  name: String,
  verts: Vec<Vec3<u32>>,
  // currently unused?
  normals: Vec<Vec3<u32>>,
  textures: Vec<Vec3<u32>>,
}

impl FaceGroup {
  pub fn new() -> Self { Default::default() }
  pub fn is_empty(&self) -> bool { self.verts.is_empty() }
}

#[derive(Debug, Default)]
pub struct IndexedTriangles {
  /// list of vertices
  verts: Vec<Vec3>,
  /// list of normals
  norms: Vec<Vec3>,
  /// list of textures
  textures: Vec<Vec3>,

  groups: Vec<FaceGroup>,
}

impl IndexedTriangles {
  pub fn len(&self) -> usize { self.groups.iter().map(|fg| fg.verts.len()).sum() }
  pub fn is_empty(&self) -> bool { self.groups.is_empty() }
  pub fn iter(&self) -> impl Iterator<Item = Triangle> + '_ {
    self.groups.iter().flat_map(move |group| {
      // TODO use normals and textures here
      group
        .verts
        .iter()
        .map(move |idxs| Triangle(idxs.apply_fn(|i| self.verts[i as usize])))
    })
  }
}

impl Shape for IndexedTriangles {
  fn intersect_ray(&self, r: &Ray) -> Option<SurfaceInteraction> {
    self
      .iter()
      .filter_map(|t| t.intersect_ray(r))
      .min_by(|a, b| a.it.closer(&b.it))
  }
}
impl Bounded for IndexedTriangles {
  fn bounds(&self) -> Bounds3 {
    let mut verts = self.verts.iter();
    let first = verts
      .next()
      .expect("Empty IndexedTriangles does not have well defined bounds");
    verts.fold(Bounds3::empty(*first), |acc, n| acc.union_vec(n))
  }
}

fn parse_slashed<D: FromStr>(s: &str) -> Result<(D, Option<D>, Option<D>), <D as FromStr>::Err>
where
  <D as FromStr>::Err: std::fmt::Debug, {
  let mut items = s.split('/');
  let v = items.next().unwrap().parse::<D>()?;
  let vt = items.next().map(|vt| vt.parse()).transpose()?;
  let vn = items.next().map(|vn| vn.parse()).transpose()?;
  Ok((v, vt, vn))
}

#[test]
fn test_from_ascii_obj() {
  let p = Path::new(file!())
    .parent()
    .unwrap()
    .join("sample_files")
    .join("teapot.obj");
  assert!(from_ascii_obj::<_, f32>(p).is_ok());
}

// Added test for more features of obj
#[test]
#[cfg(not(debug_assertions))]
fn test_from_ascii_obj_complex() {
  let p = Path::new(file!())
    .parent()
    .unwrap()
    .join("sample_files")
    .join("sponza.obj");
  assert!(from_ascii_obj::<_, f32>(p).is_ok());
}

pub fn from_ascii_stl(p: impl AsRef<Path>) -> io::Result<IndexedTriangles> {
  let f = File::open(p.as_ref())?;
  let buf = io::BufReader::new(f);
  let mut triangle_list = IndexedTriangles::default();
  let mut count = 0;
  let mut starting_index = 0;
  let mut curr_group = FaceGroup::new();
  for line in buf.lines() {
    let line = line?;
    let parts = line.split_whitespace().collect::<Vec<_>>();
    match parts.as_slice() {
      [] | [""] => (),
      ["solid", name] => curr_group.name = name.to_string(),
      ["outer", "loop"] => (),
      ["endloop"] => {
        assert_eq!(count, 3, "Expect only 3 vertices in STL file");
        let indeces = Vec3::new(starting_index, starting_index + 1, starting_index + 2);
        curr_group.verts.push(indeces);
        starting_index = triangle_list.verts.len() as u32;
        count = 0;
      },
      ["endfacet"] => assert_eq!(count, 0),
      ["endsolid", name] => assert_eq!(name, &curr_group.name),
      ["facet", "normal", n_i, n_j, n_k] => triangle_list
        .norms
        .push(Vec3::from_str_radix([n_i, n_j, n_k], 10).unwrap()),
      ["vertex", v_i, v_j, v_k] => {
        count += 1;
        triangle_list
          .verts
          .push(Vec3::from_str_radix([v_i, v_j, v_k], 10).unwrap());
      },
      _ => panic!("Unknown line while parsing ASCII STL: {:?}", line),
    };
  }
  triangle_list.groups.push(curr_group);
  Ok(triangle_list)
}

pub fn from_ascii_obj(p: impl AsRef<Path>, load_mtls: bool) -> io::Result<IndexedTriangles> {
  let f = File::open(p.as_ref())?;
  let buf = io::BufReader::new(f);
  let mut triangle_list = IndexedTriangles::default();
  let mut curr_group = FaceGroup::new();
  for line in buf.lines() {
    let line = line?;
    // TODO convert this into not using collect as it allocates
    let parts = line.split_whitespace().collect::<Vec<_>>();
    match parts.as_slice() {
      [] | ["#", ..] => (),
      ["g", name] =>
        if curr_group.is_empty() && curr_group.name.is_empty() {
          curr_group.name = name.to_string();
        } else {
          let done_group = std::mem::replace(&mut curr_group, FaceGroup::new());
          triangle_list.groups.push(done_group);
          curr_group.name = name.to_string();
        },
      ["usemtl", _mat_name] =>
        if load_mtls {
          todo!()
        },
      ["mtllib", _mtl_files @ ..] =>
        if load_mtls {
          todo!()
        },
      ["v", x, y, z] => triangle_list
        .verts
        .push(Vec3::from_str_radix([x, y, z], 10).unwrap()),
      ["v", x, y, z, _w] => triangle_list
        .verts
        .push(Vec3::from_str_radix([x, y, z], 10).unwrap()),
      // Vertex normal
      ["vn", i, j, k] => triangle_list
        .norms
        .push(Vec3::from_str_radix([i, j, k], 10).unwrap()),
      // Vertex Textures
      ["vt", u, v, w] => triangle_list
        .textures
        .push(Vec3::from_str_radix([u, v, w], 10).unwrap()),
      // Points
      ["p", _vs @ ..] => todo!(),
      // Faces
      ["f", fs @ ..] => {
        assert!(
          fs.len() >= 3,
          "OBJ faces require at least 3 elements: {:?}",
          line
        );
        let vert_indeces = fs.iter().map(|f| parse_slashed::<u32>(f).unwrap());
        for v in triangulate(vert_indeces) {
          let Vector([vs0, vs1, vs2]) = v;
          let (v0, vt0, vn0) = vs0;
          let (v1, vt1, vn1) = vs1;
          let (v2, vt2, vn2) = vs2;
          curr_group.verts.push(Vec3::new(v0, v1, v2) - 1);
          match (vt0, vt1, vt2) {
            (None, None, None) => (),
            (Some(vt0), Some(vt1), Some(vt2)) => {
              curr_group.verts.push(Vec3::new(vt0, vt1, vt2) - 1);
            },
            _ => panic!(
              "Partially specified some texture indeces but not all in OBJ file {}",
              line
            ),
          };
          match (vn0, vn1, vn2) {
            (None, None, None) => (),
            (Some(vn0), Some(vn1), Some(vn2)) => {
              curr_group.verts.push(Vec3::new(vn0, vn1, vn2) - 1);
            },
            _ => panic!(
              "Partially specified some texture indeces but not all in OBJ file {}",
              line
            ),
          };
        }
      },
      ["s", "off"] => (),
      l => panic!("Unexpected {:?}", l),
    };
  }
  if !curr_group.is_empty() {
    triangle_list.groups.push(curr_group);
  }
  Ok(triangle_list)
}

#[test]
fn test_from_ascii_stl() {
  let p = Path::new(file!())
    .parent()
    .unwrap()
    .join("sample_files")
    .join("magnolia.stl");
  assert!(from_ascii_stl::<_, f32>(p).is_ok());
}
