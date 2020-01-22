use crate::{material::Material, vec::Vec3};
use std::{io, path::Path};

#[derive(Clone)]
pub struct IndexedTriangles<'m, T> {
  // lists of vertices
  verts: Vec<Vec3<T>>,
  // list of indeces triangles
  triangles: Vec<Vec3<usize>>,

  mat: &'m dyn Material<T>,
}

#[derive(Clone, Copy)]
pub struct LoanedTriangle<'m, 'a, T> {
  pub(crate) src: &'a IndexedTriangles<'m, T>,
  pub(crate) n: usize,
}

impl<'m, 'a, T> LoanedTriangle<'m, 'a, T> {
  pub fn vert(&self, n: u8) -> &'a Vec3<T> {
    assert!(n < 3);
    &self.src.verts[*self.src.triangles[self.n].nth(n)]
  }
  pub fn mat(&self) -> &'m dyn Material<T> { (*self.src).mat }
}

pub fn from_ascii_stl<P: AsRef<Path>, T: std::str::FromStr>(
  p: P,
  mat: &dyn Material<T>,
) -> io::Result<IndexedTriangles<'_, T>> {
  use std::{fs::File, io::BufRead};
  let f = File::open(p)?;
  let buf = io::BufReader::new(f);
  let mut triangle_list: IndexedTriangles<T> = IndexedTriangles {
    verts: vec![],
    triangles: vec![],
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
        assert_eq!(count, 3, "Did not get correct count");
        let indeces = Vec3(starting_index, starting_index + 1, starting_index + 2);
        triangle_list.triangles.push(indeces);
        starting_index = triangle_list.verts.len();
        count = 0;
      },
      ["endfacet"] => assert_eq!(count, 0),
      [""] => (),
      ["endsolid", _name] => (),

      // TODO unimplemented surface normal
      ["facet", "normal", _n_i, _n_j, _n_k] => (),
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

impl<'m, T> IndexedTriangles<'m, T> {
  pub fn len(&self) -> usize { self.triangles.len() }
  pub fn iter(&self) -> Iter<'m, '_, T> {
    Iter {
      triangle_list: self,
      curr: 0,
    }
  }
  pub fn triangle(&self, n: usize) -> LoanedTriangle<'m, '_, T> { LoanedTriangle { src: self, n } }
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

#[test]
fn test() {
  use crate::material::Dielectric;
  let p = Path::new(file!());
  let p = p.parent().unwrap();
  let p = p.join("test_data").join("magnolia.stl");
  let mat = Dielectric::new(3.0);
  assert!(from_ascii_stl(p, &mat).is_ok());
}
