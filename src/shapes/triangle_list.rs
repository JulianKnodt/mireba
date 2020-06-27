use quick_maths::{Ray, Vec3, Float};
use crate::{
  mtl::{read_mtl, MTL},
  triangle::Triangle,
  util::triangulate,
  vis::{Visibility, Visible},
  bounds::{Bounded, Bounds},
};
use std::{ffi::OsString, io::BufRead, str::FromStr};

/// A group of faces
#[derive(Debug, Default)]
struct FaceGroup {
  name: String,
  /// Which material is this face using
  mtl: Option<usize>,
  verts: Vec<Vec3<u32>>,
  normals: Option<Vec<Vec3<u32>>>,
  textures: Option<Vec<Vec3<u32>>>,
}

impl FaceGroup {
  fn empty() -> Self { Default::default() }
  fn is_empty(&self) -> bool { self.verts.is_empty() }
  fn add<I: Iterator<Item = Vec3<(usize, Option<usize>, Option<usize>)>>>(&mut self, iter: I) {
    for i in iter {
      let Vec3(v1, v2, v3) = i;
      let (v1, vt1, vn1) = v1;
      let (v2, vt2, vn2) = v2;
      let (v3, vt3, vn3) = v3;
      self.verts.push(Vec3(v1, v2, v3));
      match (vt1, vt2, vt3) {
        (None, None, None) => (),
        (Some(vt1), Some(vt2), Some(vt3)) => self
          .textures
          .get_or_insert_with(Vec::new)
          .push(Vec3(vt1, vt2, vt3)),
        _ => panic!("Some vertices specified textures while others did not"),
      };
      match (vn1, vn2, vn3) {
        (None, None, None) => (),
        (Some(vn1), Some(vn2), Some(vn3)) => self
          .normals
          .get_or_insert_with(Vec::new)
          .push(Vec3(vn1, vn2, vn3)),
        _ => panic!("Some vertices specified textures while others did not"),
      }
    }
  }
}

#[derive(Debug)]
pub struct IndexedTriangles {
  /// list of vertices
  verts: Vec<Vec3>,
  /// list of normals
  norms: Vec<Vec3>,
  /// list of textures
  textures: Vec<Vec3>,

  groups: Vec<FaceGroup>,

  /// These are all MTLs cast to Mats
  // mtls: Vec<MTL>,
}

#[derive(Debug, Clone, Copy)]
pub struct LoanedTriangle<'a, T> {
  pub(crate) src: &'a IndexedTriangles<T>,
  // Which face group does this triangle belong to?
  pub(crate) group: usize,
  // Which number in the face group is this triangle
  pub(crate) n: usize,
}

/*
impl<T: Float> Bounded<Vec3<T>> for IndexedTriangles<T> {
  fn bounds(&self) -> Bounds<Vec3<T>> {
    let mut all_bounds = self.iter().map(|t| t.triangle().bounds());
    let first = all_bounds.next().unwrap();
    all_bounds.fold(first, |acc, n| acc.union(&n))
  }
}
*/

impl<'a, T: Float> Visible<T> for LoanedTriangle<'a, T> {
  fn hit(&self, r: &Ray<T>) -> Option<Visibility<T>> { self.triangle().intersect2(&r) }
}

impl<'a, T: Float> LoanedTriangle<'a, T> {
  pub fn triangle(&self) -> Triangle<&'a Vec3<T>> {
    let &Vec3(i0, i1, i2) = &self.src.groups[self.group].verts[self.n];
    let v0 = &self.src.verts[i0];
    let v1 = &self.src.verts[i1];
    let v2 = &self.src.verts[i2];
    Triangle(Vec3(v0, v1, v2))
  }
  pub fn mtl(&self) -> Option<&'a MTL<T>> {
    self.src.groups[self.group].mtl.map(|i| &self.src.mtls[i])
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Iter<'a, T> {
  triangle_list: &'a IndexedTriangles<T>,
  group: usize,
  curr: usize,
}

impl<T> IndexedTriangles<T> {
  pub fn len(&self) -> usize { self.groups.iter().map(|fg| fg.verts.len()).sum() }
  pub fn is_empty(&self) -> bool { self.groups.is_empty() }
  pub fn iter(&self) -> Iter<'_, T> {
    Iter {
      triangle_list: self,
      group: 0,
      curr: 0,
    }
  }
}

impl<'a, T> Iterator for Iter<'a, T>
where
  T: Clone,
{
  type Item = LoanedTriangle<'a, T>;
  fn next(&mut self) -> Option<Self::Item> {
    while self.curr == self.triangle_list.groups[self.group].verts.len() {
      self.group += 1;
      self.curr = 0;
      if self.group == self.triangle_list.groups.len() {
        return None;
      }
    }
    let out = LoanedTriangle {
      src: self.triangle_list,
      group: self.group,
      n: self.curr,
    };
    self.curr += 1;
    Some(out)
  }
}

pub fn parse_slashed<D: FromStr>(
  s: &str,
) -> Result<(D, Option<D>, Option<D>), <D as FromStr>::Err>
where
  <D as FromStr>::Err: std::fmt::Debug, {
  let mut items = s.split('/');
  let v = items.next().unwrap().parse::<D>()?;
  let vt = if let Some(vt) = items.next() {
    Some(vt.parse()?)
  } else {
    None
  };
  let vn = if let Some(vn) = items.next() {
    Some(vn.parse()?)
  } else {
    None
  };
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

/*
pub fn from_ascii_stl<P: AsRef<Path>, T: Float>(p: P) -> io::Result<IndexedTriangles<T>> {
  let f = File::open(p.as_ref())?;
  let buf = io::BufReader::new(f);
  let mut triangle_list: IndexedTriangles<T> = IndexedTriangles {
    verts: vec![],
    triangles: vec![],
    textures: vec![],
    norms: vec![],
    mtls: vec![],
    src_file: p.as_ref().as_os_str().to_os_string(),
  };
  let mut count = 0;
  let mut starting_index = 0;
  for line in buf.lines() {
    let line = line?;
    let parts = line.split_whitespace().collect::<Vec<_>>();
    match parts.as_slice() {
      // TODO handle name separately?
      ["solid", _name] => (),
      ["outer", "loop"] => (),
      ["endloop"] => {
        assert_eq!(count, 3, "Did not get correct count for vertices");
        let indeces = Vec3(starting_index, starting_index + 1, starting_index + 2);
        triangle_list.triangles.push(indeces);
        starting_index = triangle_list.verts.len();
        count = 0;
      },
      ["endfacet"] => assert_eq!(count, 0),
      [""] => (),
      ["endsolid", _name] => (),

      // TODO unimplemented surface normal
      ["facet", "normal", n_i, n_j, n_k] => {
        let norm = Vec3::<T>::from_str_radix((n_i, n_j, n_k), 10).unwrap_or_else(|_| todo!());
        triangle_list.norms.push(norm);
      },
      ["vertex", v_i, v_j, v_k] => {
        let v_pos = Vec3::<T>::from_str_radix((v_i, v_j, v_k), 10).unwrap_or_else(|_| todo!());
        count += 1;
        triangle_list.verts.push(v_pos);
      },
      _ => eprintln!("Unknown line while parsing ASCII STL: {:?}", line),
    }
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
*/
