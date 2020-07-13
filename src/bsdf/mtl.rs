/// Handling parsing mtl files
use super::BSDF;
use crate::{
  interaction::SurfaceInteraction,
  spectrum::{from_rgb, Spectrum},
};
use num::Num;
use quick_maths::{One, Vec3, Zero};
use std::{
  io::{self, BufRead, Read},
  mem::replace,
};

/// Directly loaded mtl file
#[derive(Debug, Clone, PartialEq)]
pub struct MTL {
  pub name: String,
  n_s: f32,
  n_i: f32,
  d: f32,
  t_r: f32,
  t_f: Vec3,
  // Illuminance kind
  pub illum: u8,
  k_ambient: Spectrum,
  k_diffuse: Spectrum,
  k_specular: Spectrum,
  k_emission: Spectrum,
}

impl MTL {
  pub fn empty() -> Self {
    Self {
      name: Default::default(),
      n_s: 0.0,
      n_i: 0.0,
      d: 0.0,
      t_r: 0.0,
      t_f: Vec3::zero(),
      illum: 0,
      k_ambient: Vec3::zero(),
      k_diffuse: Vec3::zero(),
      k_specular: Vec3::zero(),
      k_emission: Vec3::zero(),
    }
  }
  // Builder for MTL
  pub fn ambient(self, k_ambient: Vec3) -> Self { Self { k_ambient, ..self } }
  pub fn diffuse(self, k_diffuse: Vec3) -> Self { Self { k_diffuse, ..self } }
  pub fn specular(self, k_specular: Vec3) -> Self { Self { k_specular, ..self } }
}

macro_rules! quint {
  ($v: expr) => {{
    let a = $v * $v;
    a * a * $v
  }};
}

fn schlick(cos_theta: f32, n_s: f32) -> Spectrum {
  (Spectrum::one() - n_s) * quint!(1.0 - cos_theta) + n_s
}

impl BSDF for MTL {
  fn eval(&self, si: &SurfaceInteraction, wo: Vec3) -> Spectrum {
    // http://paulbourke.net/dataformats/mtl/
    // These are best guess approximations because these are not light vectors but outgoing
    // direction vectors.
    match self.illum {
      0 => self.k_diffuse,
      1 => self.k_diffuse * si.normal.dot(&wo),
      // This always includes a term for recursive ray tracing.
      2 | 3 | 4 =>
        self.k_diffuse * si.normal.dot(&wo) + self.k_ambient * (wo.dot(&si.wi.reflect(&si.normal))),
      5 => {
        let bisector = si.wi.reflect(&si.normal);
        self.k_diffuse * si.normal.dot(&wo)
          + self.k_ambient * wo.dot(&bisector) * schlick(wo.dot(&bisector), self.n_s)
          + schlick(si.normal.dot(&si.wi), self.n_s)
      },
      _ => todo!(),
    }
  }
}

/// Reads an mtl file from src, and panicks if the read failes
pub fn read_mtl(src: impl Read, out: &mut Vec<MTL>) -> io::Result<()> {
  let buf = io::BufReader::new(src);
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
          out.push(replace(&mut curr, MTL::empty()));
          curr.name = (*name).to_string();
        },
      // TODO convert below into errors
      ["Ns", ns] => curr.n_s = f32::from_str_radix(ns, 10).unwrap(),
      ["Ni", ni] => curr.n_i = f32::from_str_radix(ni, 10).unwrap(),
      ["d", d] => curr.d = f32::from_str_radix(d, 10).unwrap(),
      ["Tr", tr] => curr.t_r = f32::from_str_radix(tr, 10).unwrap(),
      ["Tf", x, y, z] => curr.t_f = Vec3::from_str_radix([x, y, z], 10).unwrap(),
      ["illum", il] => curr.illum = il.parse().unwrap(),
      ["Ka", x, y, z] => curr.k_ambient = from_rgb(Vec3::from_str_radix([x, y, z], 10).unwrap()),
      ["Kd", x, y, z] => curr.k_diffuse = from_rgb(Vec3::from_str_radix([x, y, z], 10).unwrap()),
      ["Ks", x, y, z] => curr.k_specular = from_rgb(Vec3::from_str_radix([x, y, z], 10).unwrap()),
      ["Ke", x, y, z] => curr.k_emission = from_rgb(Vec3::from_str_radix([x, y, z], 10).unwrap()),
      // TODO implement the below
      ["map_Ka", _src] => todo!(),
      ["map_Kd", _src] => todo!(),
      ["map_bump", _src] | ["bump", _src] => todo!(),
      unknown => panic!("Unknown mtl file command {:?}", unknown),
    }
  }
  Ok(())
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
