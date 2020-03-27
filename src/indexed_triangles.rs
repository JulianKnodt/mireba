use crate::{
  util::triangulate,
  vec::{Ray, Vec3, Vector},
  vis::{Visibility, Visible},
};
use num::Float;
use serde::{
  de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor},
  ser::{Serialize, SerializeStruct, Serializer},
};
use std::{ffi::OsString, io, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedTriangles<D> {
  /// list of vertices
  verts: Vec<Vec3<D>>,
  /// list of normals
  norms: Vec<Vec3<D>>,
  // /// list of textures
  // list of indeces triangles
  triangles: Vec<Vec3<usize>>,

  /// Which file did this come from?
  src_file: OsString,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoanedTriangle<'a, T> {
  pub(crate) src: &'a IndexedTriangles<T>,
  pub(crate) n: usize,
}

impl<'a, T> LoanedTriangle<'a, T> {
  pub fn vert(&self, n: u8) -> &'a Vec3<T> {
    assert!(n < 3);
    &self.src.verts[*self.src.triangles[self.n].nth(n)]
  }
  pub fn stl_normal(&self) -> Option<&'a Vec3<T>> {
    let normal_idx = self.src.triangles[self.n][0] / 3;
    self.src.norms.get(normal_idx)
  }
}

impl<'a, T: Float> LoanedTriangle<'a, T> {
  pub fn bary_normal(&self, at: &Vec3<T>) -> Vec3<T> {
    let Vec3(n0, n1, n2) = self.src.triangles[self.n];
    let n0 = &self.src.norms[n0];
    let n1 = &self.src.norms[n1];
    let n2 = &self.src.norms[n2];
    barycentric(n0, n1, n2, at)
  }
}

fn barycentric<T: Float>(a: &Vec3<T>, b: &Vec3<T>, c: &Vec3<T>, p: &Vec3<T>) -> Vec3<T> {
  let v0 = b - a;
  let v1 = c - a;
  let v2 = p - a;
  let d00 = v0.dot(&v0);
  let d01 = v0.dot(&v1);
  let d11 = v1.dot(&v1);
  let d20 = v2.dot(&v0);
  let d21 = v2.dot(&v1);
  let denom = d00 * d11 - d01 * d01;
  let v = (d11 * d20 - d01 * d21) / denom;
  let w = (d00 * d21 - d01 * d20) / denom;
  let u = T::one() - v - w;
  Vec3(u, v, w)
}

// Intersection of a triangle
impl<'a, T: Float> Visible<T> for LoanedTriangle<'a, T> {
  fn hit(&self, r: &Ray<T>) -> Option<Visibility<T>> {
    let vert0 = *self.vert(0);
    let vert1 = *self.vert(1);
    let vert2 = *self.vert(2);
    let edge_0 = vert1 - vert0;
    let edge_1 = vert2 - vert0;
    let h = r.dir.cross(&edge_1);
    let a = edge_0.dot(&h);
    if a.abs() < T::epsilon() {
      return None;
    }
    let f = a.recip();
    let s = r.pos - vert0;
    let u = f * s.dot(&h);
    if u < T::zero() || u > T::one() {
      return None;
    }
    let q = s.cross(&edge_0);
    let v = f * r.dir.dot(&q);
    if v < T::zero() || u + v > T::one() {
      return None;
    }
    let t = f * edge_1.dot(&q);
    if t < T::epsilon() || t > T::max_value() {
      return None;
    }
    // let pos = r.at(t);
    let norm = edge_0.cross(&edge_1);
    // let norm = self
    //      .bary_normal(&pos);
    // .map(|v| *v)
    // .unwrap_or_else(|| edge_0.cross(&edge_1));
    Some(Visibility {
      param: t,
      pos: r.at(t),
      norm,
    })
  }
}

impl<T> IndexedTriangles<T> {
  pub fn len(&self) -> usize { self.triangles.len() }
  pub fn is_empty(&self) -> bool { self.triangles.is_empty() }
  pub fn iter(&self) -> Iter<'_, T> {
    Iter {
      triangle_list: self,
      curr: 0,
    }
  }
  pub fn triangle(&self, n: usize) -> LoanedTriangle<'_, T> { LoanedTriangle { src: self, n } }
}

impl<T: Float> IndexedTriangles<T> {
  pub fn shift(&mut self, xyz: Vec3<T>) {
    self.verts.iter_mut().for_each(|vert| {
      vert.0 = vert.0 + xyz.0;
      vert.1 = vert.1 + xyz.1;
      vert.2 = vert.2 + xyz.2;
    });
  }
  pub fn scale(&mut self, s: T) {
    self.verts.iter_mut().for_each(|vert| {
      vert.0 = vert.0 * s;
      vert.1 = vert.1 * s;
      vert.2 = vert.2 * s;
    });
  }
}

pub struct Iter<'a, T> {
  triangle_list: &'a IndexedTriangles<T>,
  curr: usize,
}

impl<'a, T> Iterator for Iter<'a, T>
where
  T: Clone,
{
  type Item = LoanedTriangle<'a, T>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.curr == self.triangle_list.len() {
      return None;
    }
    let out = self.triangle_list.triangle(self.curr);
    self.curr += 1;
    Some(out)
  }
}

use std::{fs::File, io::BufRead};

pub fn from_ascii_stl<P: AsRef<Path>, T: Float>(p: P) -> io::Result<IndexedTriangles<T>> {
  let f = File::open(p.as_ref())?;
  let buf = io::BufReader::new(f);
  let mut triangle_list: IndexedTriangles<T> = IndexedTriangles {
    verts: vec![],
    triangles: vec![],
    norms: vec![],
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
  assert!(from_ascii_stl(p, crate::material::CHECKERS_REF).is_ok());
}

pub fn from_ascii_obj<P: AsRef<Path>, T: Float>(p: P) -> io::Result<IndexedTriangles<T>> {
  let f = File::open(p.as_ref())?;
  let buf = io::BufReader::new(f);
  let mut triangle_list = IndexedTriangles {
    verts: vec![],
    triangles: vec![],
    norms: vec![],
    src_file: p.as_ref().as_os_str().to_os_string(),
  };
  for line in buf.lines() {
    let line = line?;
    // TODO convert this into not using convert
    let parts = line.split_whitespace().collect::<Vec<_>>();
    match parts.as_slice() {
      [] | ["#", ..] => (),
      // TODO figure out what this means
      ["g", ..] => (),
      ["v", x, y, z] => {
        let pos = Vec3::<T>::from_str_radix((x, y, z), 10).unwrap_or_else(|_| todo!());
        triangle_list.verts.push(pos);
      },
      ["v", x, y, z, _w] => {
        let pos = Vec3::<T>::from_str_radix((x, y, z), 10).unwrap_or_else(|_| todo!());
        triangle_list.verts.push(pos);
      },
      // Vertex normal
      ["vn", i, j, k] => {
        let norm = Vec3::<T>::from_str_radix((i, j, k), 10).unwrap_or_else(|_| todo!());
        triangle_list.norms.push(norm);
      },
      // Vertex Textures
      ["vt", _u, _v, _w] => todo!(),
      // Points
      ["p", _vs @ ..] => todo!(),
      // Faces
      ["f", fs @ ..] => {
        if fs.len() < 3 {
          panic!("OBJ faces require at least 3 elements")
        }
        let vert_indeces = fs
          .iter()
          .map(|f| f.parse::<usize>().unwrap())
          // 1 indexed in obj so need to subtract 1
          .map(|f| f - 1);
        triangulate(vert_indeces).for_each(|f| {
          triangle_list.triangles.push(f);
        });
      },
      ["s", "off"] => (),
      l => panic!("Unexpected {:?}", l),
    };
  }
  Ok(triangle_list)
}

impl<D> Serialize for IndexedTriangles<D> {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    let mut state = serializer.serialize_struct("IndexedTriangles", 1)?;
    state.serialize_field("src_file", &self.src_file)?;
    state.end()
  }
}

use std::fmt;
impl<'de, T: Float> Deserialize<'de> for IndexedTriangles<T> {
  fn deserialize<D: Deserializer<'de>>(des: D) -> Result<Self, D::Error> {
    use std::marker::PhantomData;
    #[derive(Debug, serde::Deserialize)]
    enum Field {
      #[serde(rename = "src_file")]
      SrcFile,
    }
    struct ITVisitor<F>(PhantomData<F>);
    impl<'de, F: Float> Visitor<'de> for ITVisitor<F> {
      type Value = IndexedTriangles<F>;
      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct IndexedTrianges")
      }
      fn visit_seq<V: SeqAccess<'de>>(self, mut seq: V) -> Result<Self::Value, V::Error> {
        let p: OsString = seq
          .next_element()?
          .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let idt = match Path::new(&p).extension() {
          None => panic!("No extension, cannot render"),
          Some(stl) if stl == "stl" => from_ascii_stl(stl),
          Some(obj) if obj == "obj" => from_ascii_obj(p),
          v => panic!("Invalid extension {:?}", v),
        };
        Ok(idt.unwrap())
      }
      fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let p: OsString = if let Some(Field::SrcFile) = map.next_key::<Field>()? {
          map.next_value()?
        } else {
          return Err(de::Error::missing_field("src_file"));
        };
        let idt = match Path::new(&p).extension() {
          None => panic!("No extension, cannot render"),
          Some(stl) if stl == "stl" => from_ascii_stl(stl),
          Some(obj) if obj == "obj" => from_ascii_obj(p),
          v => panic!("Invalid extension {:?}", v),
        };
        Ok(idt.unwrap())
      }
    }
    const FIELDS: &'static [&'static str] = &["src_file"];
    des.deserialize_struct("IndexedTriangles", FIELDS, ITVisitor(PhantomData))
  }
}

#[test]
fn test_from_ascii_obj() {
  let p = Path::new(file!())
    .parent()
    .unwrap()
    .join("sample_files")
    .join("teapot.obj");
  assert!(from_ascii_obj(p).is_ok());
}
