use crate::{
  material::Mat,
  vec::{Ray, Vec3},
  vis::{Visibility, Visible},
};
use num::Float;
use std::{io, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedTriangles<'m, D> {
  /// list of vertices
  verts: Vec<Vec3<D>>,
  /// list of normals
  norms: Vec<Vec3<D>>,
  // /// list of textures
  // list of indeces triangles
  triangles: Vec<Vec3<usize>>,

  mat: &'m Mat<D>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoanedTriangle<'m, 'a, T> {
  pub(crate) src: &'a IndexedTriangles<'m, T>,
  pub(crate) n: usize,
}

impl<'m, 'a, T> LoanedTriangle<'m, 'a, T> {
  pub fn vert(&self, n: u8) -> &'a Vec3<T> {
    assert!(n < 3);
    &self.src.verts[*self.src.triangles[self.n].nth(n)]
  }
  pub fn stl_normal(&self) -> Option<&'a Vec3<T>> {
    let normal_idx = self.src.triangles[self.n][0] / 3;
    self.src.norms.get(normal_idx)
  }
}

impl<'m, 'a, T: Clone> LoanedTriangle<'m, 'a, T> {
  /// Returns the material for this triangle
  #[inline]
  pub fn mat(&self) -> &'m Mat<T> { self.src.mat }
}

fn barycentric<T: Float>(a: Vec3<T>, b: Vec3<T>, c: Vec3<T>, p: Vec3<T>) -> Vec3<T> {
  let v0 = b - a;
  let v1 = c - a;
  let v2 = p - a;
  let d00 = v0.dot(v0);
  let d01 = v0.dot(v1);
  let d11 = v1.dot(v1);
  let d20 = v2.dot(v0);
  let d21 = v2.dot(v1);
  let denom = d00 * d11 - d01 * d01;
  let v = (d11 * d20 - d01 * d21) / denom;
  let w = (d00 * d21 - d01 * d20) / denom;
  let u = T::one() - v - w;
  Vec3(u, v, w)
}

// Intersection of a triangle
impl<'m, 'a, T: Float> Visible<'m, T> for LoanedTriangle<'m, 'a, T> {
  fn hit(&self, r: &Ray<T>) -> Option<Visibility<'m, T>> {
    let epsilon = T::from(1e-6).unwrap();
    let vert0 = *self.vert(0);
    let vert1 = *self.vert(1);
    let vert2 = *self.vert(2);
    let edge_0 = vert1 - vert0;
    let edge_1 = vert2 - vert0;
    let h = r.dir.cross(edge_1);
    let a = edge_0.dot(h);
    if a > -epsilon && a < epsilon {
      return None;
    }
    let f = a.recip();
    let s = r.pos - vert0;
    let u = f * s.dot(h);
    if u < T::zero() || u > T::one() {
      return None;
    }
    let q = s.cross(edge_0);
    let v = f * r.dir.dot(q);
    if v < T::zero() || u + v > T::one() {
      return None;
    }
    let t = f * edge_1.dot(q);
    if !(t > epsilon && t < epsilon.recip()) {
      return None;
    }
    let norm = self
      // temporarily here
      .stl_normal()
      .map(|v| *v)
      .unwrap_or_else(|| edge_0.cross(edge_1));
    Some(Visibility {
      param: t,
      pos: r.at(t),
      norm,
      mat: self.mat(),
    })
  }
}

impl<'m, T> IndexedTriangles<'m, T> {
  pub fn len(&self) -> usize { self.triangles.len() }
  pub fn is_empty(&self) -> bool { self.triangles.is_empty() }
  pub fn iter(&self) -> Iter<'m, '_, T> {
    Iter {
      triangle_list: self,
      curr: 0,
    }
  }
  pub fn triangle(&self, n: usize) -> LoanedTriangle<'m, '_, T> { LoanedTriangle { src: self, n } }
}

impl<'m, T: Float> IndexedTriangles<'m, T> {
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

pub struct Iter<'m, 'a, T> {
  triangle_list: &'a IndexedTriangles<'m, T>,
  curr: usize,
}

impl<'m, 'a, T> Iterator for Iter<'m, 'a, T>
where
  T: Clone,
{
  type Item = LoanedTriangle<'m, 'a, T>;
  fn next(&mut self) -> Option<Self::Item> {
    if self.curr == self.triangle_list.len() {
      return None;
    }
    let out = self.triangle_list.triangle(self.curr);
    self.curr += 1;
    Some(out)
  }
}

use std::{fs::File, io::BufRead, str::FromStr};

pub fn from_ascii_stl<P: AsRef<Path>, T: FromStr>(
  p: P,
  mat: &Mat<T>,
) -> io::Result<IndexedTriangles<'_, T>> {
  let f = File::open(p)?;
  let buf = io::BufReader::new(f);
  let mut triangle_list: IndexedTriangles<T> = IndexedTriangles {
    verts: vec![],
    triangles: vec![],
    norms: vec![],
    mat,
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
        let n_i = n_i.parse::<T>().unwrap_or_else(|_| todo!());
        let n_j = n_j.parse::<T>().unwrap_or_else(|_| todo!());
        let n_k = n_k.parse::<T>().unwrap_or_else(|_| todo!());
        triangle_list.norms.push(Vec3(n_i, n_j, n_k));
      },
      ["vertex", v_i, v_j, v_k] => {
        let v_i = v_i.parse::<T>().unwrap_or_else(|_| todo!());
        let v_j = v_j.parse::<T>().unwrap_or_else(|_| todo!());
        let v_k = v_k.parse::<T>().unwrap_or_else(|_| todo!());
        count += 1;
        triangle_list.verts.push(Vec3(v_i, v_j, v_k));
      },
      _ => eprintln!("Unknown line while parsing ASCII STL: {:?}", line),
    }
  }
  Ok(triangle_list)
}

#[test]
fn test_from_ascii() {
  use crate::material::Dielectric;
  let p = Path::new(file!());
  let p = p.parent().unwrap();
  let p = p.join("test_data").join("magnolia.stl");
  let mat = Dielectric::new(3.0);
  assert!(from_ascii_stl(p, MatRef::from(&mat)).is_ok());
}

pub fn from_obj<P: AsRef<Path>, T: FromStr>(
  p: P,
  mat: &Mat<T>,
) -> io::Result<IndexedTriangles<'_, T>> {
  let f = File::open(p)?;
  let buf = io::BufReader::new(f);
  let mut triangle_list = IndexedTriangles {
    verts: vec![],
    triangles: vec![],
    norms: vec![],
    mat,
  };
  for line in buf.lines() {
    let line = line?;
    // TODO convert this into not using convert
    let parts = line.split_whitespace().collect::<Vec<_>>();
    match parts.as_slice() {
      [] => (),
      ["v", x, y, z, _w] => {
        let x = x.parse::<T>().unwrap_or_else(|_| todo!());
        let y = y.parse::<T>().unwrap_or_else(|_| todo!());
        let z = z.parse::<T>().unwrap_or_else(|_| todo!());
        triangle_list.verts.push(Vec3(x, y, z));
      },
      // Vertex normal
      ["vn", _i, _j, _k] => todo!(),
      // Vertex Textures
      ["vt", _u, _v, _w] => todo!(),
      // Points
      ["p", _vs @ ..] => todo!(),
      // Faces
      ["f", fs @ ..] => {
        if fs.len() < 3 {
          panic!("OBJ faces require at least 3 elements")
        }
        // need to handle arbitrary faces by possibly making them triangles
        if fs.len() != 3 {
          todo!()
        }
        let a = fs[0].parse::<usize>().unwrap_or_else(|_| todo!());
        let b = fs[1].parse::<usize>().unwrap_or_else(|_| todo!());
        let c = fs[2].parse::<usize>().unwrap_or_else(|_| todo!());
        triangle_list.triangles.push(Vec3(a, b, c));
      },
      _ => unimplemented!(),
    };
  }
  Ok(triangle_list)
}
