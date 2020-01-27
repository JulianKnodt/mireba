extern crate image;
use crate::vec::Vec3;
use image::{ImageBuffer, Rgb};
use std::path::Path;

pub struct Screen {
  w: usize,
  h: usize,
  data: Vec<Vec3<f32>>,
}

fn vec3_to_rgb(v: &Vec3<f32>) -> Rgb<u8> { Rgb([v.0 as u8, v.1 as u8, v.2 as u8]) }

impl Screen {
  pub fn new(w: usize, h: usize) -> Self {
    let mut data = Vec::with_capacity(w * h);
    data.resize_with(w * h, Default::default);
    Screen { w, h, data }
  }
  pub fn set(&mut self, x: usize, y: usize, val: Vec3<f32>) { self.data[x + self.w * y] = val; }
  fn get(&self, x: usize, y: usize) -> &Vec3<f32> { &self.data[x + self.w * y] }
}

impl Screen {
  pub fn write_ppm(&self) {
    print!("P3\n{} {}\n255\n", self.w, self.h);
    (0..self.h).for_each(|y| {
      (0..self.w).for_each(|x| {
        let color = *self.get(x, y);
        println!("{} {} {}", color.0 as i32, color.1 as i32, color.2 as i32);
      })
    });
  }
  pub fn write_image<Q: AsRef<Path>>(&self, to: Q) {
    let mut buf = ImageBuffer::new(self.w as u32, self.h as u32);
    (0..self.h).for_each(|y| {
      (0..self.w).for_each(|x| {
        let color = vec3_to_rgb(self.get(x, y));
        buf.put_pixel(x as u32, y as u32, color);
      })
    });
    buf.save(to).expect("Failed to save");
  }
}
