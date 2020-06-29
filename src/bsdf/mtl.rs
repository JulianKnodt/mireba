/// Handling parsing mtl files
use crate::material::Material;
use linalg::num::Float;
use linalg::vec::{Ray, Vec3, Vector};
use crate::{
  brdf::Illum,
  vis::Visibility,
};
use num::Zero;
use std::{
  io::{self, BufRead, Read},
  mem::replace,
};

/// Directly loaded material loaded from an mtl file
#[derive(Debug, Clone, PartialEq)]
pub struct MTL<T> {
  pub name: String,
  n_s: T,
  n_i: T,
  d: T,
  t_r: T,
  t_f: Vec3<T>,
  // Illuminance kind
  pub illum: Illum,
  k_ambient: Vec3<T>,
  k_diffuse: Vec3<T>,
  k_specular: Vec3<T>,
  k_emission: Vec3<T>,
}

impl<T: Float> MTL<T> {
  pub(crate) fn empty() -> Self {
    Self {
      name: Default::default(),
      n_s: T::zero(),
      n_i: T::zero(),
      d: T::zero(),
      t_r: T::zero(),
      t_f: Vec3::zero(),
      illum: Illum::Checkers,
      k_ambient: Vec3::zero(),
      k_diffuse: Vec3::zero(),
      k_specular: Vec3::zero(),
      k_emission: Vec3::zero(),
    }
  }
  // Builder for MTL
  pub fn ambient(self, k_ambient: Vec3<T>) -> Self { Self { k_ambient, ..self } }
  pub fn diffuse(self, k_diffuse: Vec3<T>) -> Self { Self { k_diffuse, ..self } }
  pub fn specular(self, k_specular: Vec3<T>) -> Self { Self { k_specular, ..self } }
  pub fn illum(self, illum: u8) -> Self {
    let illum = Illum::new(illum);
    Self { illum, ..self }
  }
  pub fn brdf(self, illum: Illum) -> Self { Self { illum, ..self } }
  //
}

impl<T: Float> Material<T> for MTL<T> {
  fn diffuse_refl(&self) -> Vec3<T> { self.k_diffuse }
  fn specular_refl(&self) -> Vec3<T> { self.k_specular }
  fn ambient_refl(&self) -> Vec3<T> { self.k_ambient }
  fn shine(&self) -> T { self.n_s }
  fn transparent_refl(&self) -> Vec3<T> { self.t_f }
  fn reflected(&self, eye: &Ray<T>, vis: &Visibility<T>) -> Option<Ray<T>> {
    // TODO check illum here to determine method of reflection
    let incident = (eye.pos - vis.pos).norm();
    Some(Ray::new(vis.pos, incident.reflect(&vis.norm)))
  }
  fn refracted(&self, eye: &Ray<T>, vis: &Visibility<T>) -> Option<Ray<T>> {
    // index of refraction is n_i
    let incident = (eye.pos - vis.pos).norm();
    let flip_eta = incident.dot(&vis.norm).is_sign_negative();
    let eta = if flip_eta { -self.n_i } else { self.n_i };
    incident
      .refract(vis.norm, eta)
      .map(|dir| Ray::new(vis.pos, dir))
  }
}

/// Reads an mtl file from src, and panicks if the read failes
pub fn read_mtl<R: Read, T: Float>(src: R) -> io::Result<Vec<MTL<T>>> {
  let buf = io::BufReader::new(src);
  let mut out = vec![];
  let mut curr = MTL::empty();
  for line in buf.lines() {
    let line = line?;
    let parts = line
      .splitn(2, '#')
      .next()
      .unwrap()
      .split_whitespace()
      .collect::<Vec<_>>();
    match parts.as_slice() {
      [] => (),
      ["newmtl", name] =>
        if curr.name.is_empty() {
          curr.name = (*name).to_string();
        } else {
          let finished = replace(&mut curr, MTL::empty());
          out.push(finished);
          curr.name = (*name).to_string();
        },
      // TODO convert below into errors
      ["Ns", ns] => curr.n_s = T::from_str_radix(ns, 10).unwrap_or_else(|_| T::zero()),
      ["Ni", ni] => curr.n_i = T::from_str_radix(ni, 10).unwrap_or_else(|_| T::zero()),
      ["d", d] => curr.d = T::from_str_radix(d, 10).unwrap_or_else(|_| T::zero()),
      ["Tr", tr] => curr.t_r = T::from_str_radix(tr, 10).unwrap_or_else(|_| T::zero()),
      ["Tf", x, y, z] =>
        curr.t_f = Vec3::<T>::from_str_radix((x, y, z), 10).unwrap_or_else(|_| Vec3::zero()),
      ["illum", il] => curr.illum = Illum::new(il.parse().unwrap()),
      ["Ka", x, y, z] =>
        curr.k_ambient = Vec3::<T>::from_str_radix((x, y, z), 10).unwrap_or_else(|_| Vec3::zero()),
      ["Kd", x, y, z] =>
        curr.k_diffuse = Vec3::<T>::from_str_radix((x, y, z), 10).unwrap_or_else(|_| Vec3::zero()),
      ["Ks", x, y, z] =>
        curr.k_specular = Vec3::<T>::from_str_radix((x, y, z), 10).unwrap_or_else(|_| Vec3::zero()),
      ["Ke", x, y, z] =>
        curr.k_emission = Vec3::<T>::from_str_radix((x, y, z), 10).unwrap_or_else(|_| Vec3::zero()),
      // TODO implement the below
      ["map_Ka", _src] => (),
      ["map_Kd", _src] => (),
      ["map_bump", _src] => (),
      ["bump", _src] => (),
      unknown => panic!("Unknown mtl file command {:?}", unknown),
    }
  }
  Ok(out)
}

#[test]
fn test_mtl_load() {
  use std::{fs::File, path::Path};
  let p = Path::new(file!())
    .parent()
    .unwrap()
    .join("sample_files")
    .join("sponza.mtl");
  let r = File::open(p).unwrap();
  assert!(read_mtl::<_, f32>(r).is_ok());
}
