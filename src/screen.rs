use crate::{
  color::RGB,
};
use linalg::vec::{Vec2, Vec3};
use linalg::num::Float;
use num::Zero;
use image::{ImageBuffer, Rgb, Rgba};
use std::path::Path;

// TODO make screen generic over floats
pub struct Screen {
  w: usize,
  h: usize,
  data: Vec<Vec3<f32>>,
}

fn vec3_to_rgb(v: &Vec3<f32>) -> Rgb<u8> {
  Rgb([(v.0 * 255.) as u8, (v.1 * 255.) as u8, (v.2 * 255.) as u8])
}

fn rgb_to_rgba(c: Rgb<u8>) -> Rgba<u8> {
  let Rgb([r, g, b]) = c;
  Rgba([r, g, b, 255])
}

impl Screen {
  pub fn new(w: usize, h: usize) -> Self {
    let mut data = Vec::with_capacity(w * h);
    data.resize_with(w * h, Vec3::zero);
    Screen { w, h, data }
  }
  pub fn fill(&mut self, v: Vec3<f32>) {
    for i in 0..self.data.len() {
      self.data[i] = v;
    }
  }
  pub fn set(&mut self, x: usize, y: usize, val: Vec3<f32>) {
    if x < self.w && y < self.h {
      self.data[x + self.w * y] = val;
    }
  }
  pub fn get(&self, x: usize, y: usize) -> &Vec3<f32> { &self.data[x + self.w * y] }
  pub fn opt_get(&self, x: usize, y: usize) -> Option<&Vec3<f32>> { self.data.get(x + self.w * y) }
  pub fn line<C: Into<RGB<f32>>>(&mut self, p_0: Vec2<f32>, p_1: Vec2<f32>, color: C) {
    let (p_0, p_1) = if p_0.0 < p_1.0 {
      (p_0, p_1)
    } else {
      (p_1, p_0)
    };
    let Vec2(x_0, y_0) = p_0;
    let Vec2(x_1, y_1) = p_1;
    let val = color.into().val();
    match (x_0 as usize == x_1 as usize, y_0 as usize == y_1 as usize) {
      (true, true) => self.set(x_0 as usize, y_0 as usize, val),
      (true, false) => {
        let r = if y_0 > y_1 {
          (y_1 as usize)..(y_0 as usize)
        } else {
          (y_0 as usize)..(y_1 as usize)
        };
        for j in r {
          self.set(x_0 as usize, j, val);
        }
      },
      (false, true) =>
        for i in (x_0 as usize)..(x_1 as usize) {
          self.set(i, y_0 as usize, val);
        },
      (false, false) => {
        let m = (y_1 - y_0) / (x_1 - x_0);
        if m.abs() <= 1.0 {
          for x in (x_0 as usize)..=(x_1 as usize) {
            let y = m * (x as f32 - x_0) + y_0;
            self.set(x, y as usize, val);
          }
        } else {
          let (l, u) = if p_0.1 < p_1.1 {
            (p_0, p_1)
          } else {
            (p_1, p_0)
          };
          for y in (l.1 as usize)..=(u.1 as usize) {
            let x = (y as f32 - l.1) / m + l.0;
            self.set(x as usize, y, val);
          }
        }
      },
    }
  }
  pub fn circle<C: Into<RGB<f32>>>(&mut self, p_0: Vec2<f32>, r: f32, c: C) {
    assert!(r > 0.);
    let val = c.into().val();
    let Vec2(x, y) = p_0;
    let (lx, ux) = ((x - r).max(0.), (x + r).min(self.w as f32).max(0.));
    let r_sqr = r * r;
    for i in (lx as usize)..(ux as usize) {
      let dx = i as f32 - x;
      let dy = (r_sqr - (dx * dx)).abs().sqrt();
      let (ly, uy) = ((y - dy).max(0.), (y + dy).min(self.h as f32));
      for j in (ly as usize)..(uy as usize) {
        self.set(i, j, val);
      }
    }
  }
  // Convolves this screen with a window in the x dimension
  pub fn convolve_x(&mut self, window: &[f32]) -> Self {
    assert_eq!(window.len() % 2, 1, "Must pass a window of odd length");
    let mut out = Self::new(self.w, self.h);
    let half_len = (window.len() / 2) as isize;
    for i in 0..self.w {
      for j in 0..self.h {
        let mut sum = Vec3::zero();
        for x in -half_len..=half_len {
          if let Some(v) = self.opt_get((i as isize + x) as usize, j) {
            sum += *v * window[(half_len + x) as usize];
          }
        }
        out.set(i, j, sum);
      }
    }
    out
  }
  // Convolves this screen with a window in the x dimension
  pub fn convolve_y(&mut self, window: &[f32]) -> Self {
    assert_eq!(window.len() % 2, 1, "Must pass a window of odd length");
    let mut out = Self::new(self.w, self.h);
    let half_len = (window.len() / 2) as isize;
    for i in 0..self.w {
      for j in 0..self.h {
        let mut sum = Vec3::zero();
        for y in -half_len..=half_len {
          if let Some(v) = self.opt_get(i, (j as isize + y) as usize) {
            sum += *v * window[(half_len + y) as usize];
          }
        }
        out.set(i, j, sum);
      }
    }
    out
  }
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
    self.write_buffer().save(to).expect("Failed to save");
  }
  pub fn write_buffer_into(&self, buf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    (0..self.h).for_each(|y| {
      (0..self.w).for_each(|x| {
        let color = rgb_to_rgba(vec3_to_rgb(self.get(x, y)));
        buf.put_pixel(x as u32, y as u32, color);
      })
    });
  }
  pub fn write_buffer(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut buf = ImageBuffer::new(self.w as u32, self.h as u32);
    self.write_buffer_into(&mut buf);
    buf
  }
}

#[derive(Debug)]
pub struct ZBuffer<T: Float>(pub Vec<T>);
impl<T: Float> ZBuffer<T> {
  pub fn new(width: usize, height: usize) -> Self { ZBuffer(vec![T::one(); width * height]) }
  pub fn reset(&mut self) {
    for i in 0..self.0.len() {
      self.0[i] = T::one();
    }
  }
  #[inline]
  pub fn mark(&mut self, i: usize, v: T) -> bool {
    let check = self.0[i] < v && v >= -T::one();
    if check {
      self.0[i] = v;
    }
    check
  }
}
