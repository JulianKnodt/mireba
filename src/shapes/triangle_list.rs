use super::triangle::Triangle;
use quick_maths::{Vec3};
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
  /*
  fn add<I>(&mut self, iter: I)
  where
    I: Iterator<Item = (Vec3<u32>, Option<Vec3<u32>>, Option<Vec3<u32>>)>, {
    for (v, vt, vn) in iter {
      self.verts.push(v);
      if let Some(vt) = vt {
        self.textures.push(vt);
      }
      if let Some(vn) = vn {
        self.normals.push(vn);
      }
    }
  }
  */
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

pub fn parse_slashed<D: FromStr>(
  s: &str,
) -> Result<(D, Option<D>, Option<D>), <D as FromStr>::Err>
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
      // TODO handle name separately?
      ["solid", name] => curr_group.name = name.to_string(),
      ["outer", "loop"] => (),
      ["endloop"] => {
        assert_eq!(count, 3, "Did not get correct count for vertices");
        let indeces = Vec3::new(starting_index, starting_index + 1, starting_index + 2);
        curr_group.verts.push(indeces);
        starting_index = triangle_list.verts.len() as u32;
        count = 0;
      },
      ["endfacet"] => assert_eq!(count, 0),
      [""] => (),
      ["endsolid", name] => assert_eq!(name, &curr_group.name),
      ["facet", "normal", n_i, n_j, n_k] => {
        let norm = Vec3::from_str_radix([n_i, n_j, n_k], 10).unwrap();
        triangle_list.norms.push(norm);
      },
      ["vertex", v_i, v_j, v_k] => {
        let v_pos = Vec3::from_str_radix([v_i, v_j, v_k], 10).unwrap();
        count += 1;
        triangle_list.verts.push(v_pos);
      },
      _ => eprintln!("Unknown line while parsing ASCII STL: {:?}", line),
    };
  }
  triangle_list.groups.push(curr_group);
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
