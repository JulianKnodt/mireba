use super::Film;
use crate::{spectrum::Spectrum, utils::morton_encode};
use quick_maths::{Vec2, Vector};

impl Film {
  /// Draws a line on the film
  pub fn line(&mut self, val: Spectrum, s: Vec2, t: Vec2) {
    let s = s * self.size.apply_fn(|v| v as f32);
    let t = t * self.size.apply_fn(|v| v as f32);
    let (s, t) = if s.x() < t.x() { (s, t) } else { (t, s) };
    let Vector([x0, y0]) = s;
    let Vector([x1, y1]) = t;
    let mut storage = self.storage.write().unwrap();
    match (x0 as u32 == x1 as u32, y0 as u32 == y1 as u32) {
      (true, true) => storage[morton_encode(x0 as u32, y0 as u32) as usize] = val,
      (true, false) => {
        let r = if y0 > y1 {
          (y1 as u32)..(y0 as u32)
        } else {
          (y0 as u32)..(y1 as u32)
        };
        for j in r {
          storage[morton_encode(x0 as u32, j) as usize] = val;
        }
      },
      (false, true) =>
        for i in (x0 as u32)..(x1 as u32) {
          storage[morton_encode(i, y0 as u32) as usize] = val;
        },
      (false, false) => {
        // slope
        let m = (y1 - y0) / (x1 - x0);
        if m.abs() <= 1.0 {
          for x in (x0 as u32)..=(x1 as u32) {
            let y = m * (x as f32 - x0) + y0;
            storage[morton_encode(x, y as u32) as usize] = val;
          }
        } else {
          let (l, u) = if s.y() < t.y() { (s, t) } else { (t, s) };
          for y in (l.y() as u32)..=(u.y() as u32) {
            let x = (y as f32 - l.y()) / m + l.x();
            storage[morton_encode(x as u32, y) as usize] = val;
          }
        }
      },
    }
  }
  pub fn circle(&mut self, c: Vec2, r: f32, val: Spectrum) {
    assert!(r >= 0.);
    let Vector([x, y]) = c;
    let (lx, ux) = ((x - r).max(0.), (x + r).min(self.size.x() as f32).max(0.));
    let r_sqr = r * r;
    let mut storage = self.storage.write().unwrap();
    for i in (lx as u32)..(ux as u32) {
      let dx = i as f32 - x;
      let dy = (r_sqr - (dx * dx)).abs().sqrt();
      let (ly, uy) = ((y - dy).max(0.), (y + dy).min(self.size.y() as f32));
      for j in (ly as u32)..(uy as u32) {
        storage[morton_encode(i, j) as usize] = val;
      }
    }
  }
}
