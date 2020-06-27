pub mod blocks;
pub mod builder;
pub use builder::Builder;
pub mod draw;

use crate::{
  spectrum::{self, Spectrum},
  utils::{morton_decode, morton_encode},
};
use image::{DynamicImage, GenericImage, Rgba};
use quick_maths::{Vec2, Vec3, Vector, Zero};
use std::{fmt::Debug, sync::RwLock};

#[derive(Debug)]
pub struct Film {
  pub size: Vec2<u32>,
  // TODO replace this backend?
  storage: RwLock<Vec<Spectrum>>,
}

impl Film {
  pub fn empty(w: u32, h: u32) -> Self {
    Self {
      size: Vec2::new(w, h),
      storage: RwLock::new(vec![Spectrum::zero(); (w * h) as usize]),
    }
  }
  pub fn write(&self, uv: Vec2, val: Spectrum) {
    if val.is_zero() {
      // don't need to acquire lock if writing nothing
      return;
    }
    let Vector([x, y]) = uv * self.size.apply_fn(|v| v as f32);
    self.write_pixel((x as u32, y as u32), val);
  }
  fn write_pixel(&self, (x, y): (u32, u32), val: Spectrum) {
    self.storage.write().unwrap()[morton_encode(x, y) as usize] = val;
  }
  pub fn blocks(&self) -> Vec<ImageBlock> { blocks::naive((self.size.x(), self.size.y()), (1, 1)) }
  /// Converts this film into a dynamic image
  pub fn to_image(&self) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(self.size.x(), self.size.y());
    for (i, &v) in self.storage.read().unwrap().iter().enumerate() {
      let (x, y) = morton_decode(i as u32);
      let Vector([r, g, b]) = (spectrum::to_rgb(v) * 255.).powf(2.2).min(255.);
      img.put_pixel(x, y, Rgba([r as u8, g as u8, b as u8, 255]));
    }
    img
  }
}

/// Represents one portion of the film
#[derive(Debug)]
pub struct ImageBlock {
  // offset from top left corner
  offset: Vec2<u32>,
  // how large is this image block
  size: Vec2<u32>,
  // channels: u32, // implicitly three as seen below, can make it generic over N later
  pub data: Vec<Vec3>,
}

impl ImageBlock {
  pub fn new((w, h): (u32, u32), (x, y): (u32, u32)) -> Self {
    Self {
      offset: Vec2::new(x, y),
      size: Vec2::new(w, h),
      data: vec![Vec3::of(0.0); (w * h) as usize],
    }
  }
  pub fn write(&mut self, p: Vec2<u32>, val: Vec3) {
    let inside = p - self.offset;
    assert!(inside.x() < self.size.x());
    assert!(inside.y() < self.size.y());
    self.data[morton_encode(inside.x(), inside.y()) as usize] = val;
  }
  pub fn w(&self) -> u32 { self.size.x() }
  pub fn h(&self) -> u32 { self.size.y() }
  /// Returns positions inside this image block
  pub fn positions(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
    (0..self.w()).flat_map(move |x| {
      let x = x + self.offset.x();
      (self.offset.y()..self.offset.y() + self.h()).map(move |y| (x, y))
    })
  }
}
