use crate::vec::Vec3;
use num::Float;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorSpace {
  RGB,
  HSL,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Color<D = f32> {
  pub v: Vec3<D>,
  space: ColorSpace,
}

impl<D> Color<D> {
  pub fn rgb(r: D, g: D, b: D) -> Self {
    Color {
      v: Vec3(r, g, b),
      space: ColorSpace::RGB,
    }
  }
}

impl<D: Float> Color<D> {
  pub fn gamma(&self, gamma: D) -> Self {
    Color {
      v: self.v.apply_fn(|i| i.powf(gamma)),
      ..*self
    }
  }
  pub fn to_hsl(&self) -> Self {
    match self.space {
      ColorSpace::HSL => *self,
      ColorSpace::RGB => {
        let Vec3(r, g, b) = self.v;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let mut hsl = Vec3::from(D::zero());
        let two = D::from(2.0).unwrap();
        hsl.2 = (max + min) / two;
        if max != min {
          let d = max - min;
          hsl.1 = d
            / (if hsl.2 > D::from(0.5).unwrap() {
              two - max - min
            } else {
              max + min
            });
          let six = D::from(6.0).unwrap();
          hsl.0 = if max == r {
            (g - b) / d + if g < b { six } else { D::zero() }
          } else if max == g {
            (b - r) / d + two
          } else {
            (r - g) / d + D::from(4.0).unwrap()
          };
          hsl.0 = hsl.0 / six;
        };
        Self {
          v: hsl,
          space: ColorSpace::HSL,
        }
      },
    }
  }
  pub fn to_rgb(&self) -> Self {
    match self.space {
      ColorSpace::RGB => *self,
      ColorSpace::HSL => {
        // let Vec3(h, s, l) = self.v;
        todo!();
      },
    }
  }
  /// maps the value of this color for use in the screen module
  pub fn val(&self) -> Vec3<D> {
    let Vec3(i, j, k) = self.v;
    let max = D::from(255.0).unwrap();
    Vec3(i * max, j * max, k * max)
  }
  /// Creates a gray scale color
  pub fn tone(t: D) -> Self {
    let t = num::clamp(t, D::zero(), D::one());
    Color {
      v: Vec3(t, t, t),
      space: ColorSpace::RGB,
    }
  }
}

// naively converts Vec3 into color
impl<D: Float> From<Vec3<D>> for Color<D> {
  fn from(v: Vec3<D>) -> Self {
    Color {
      v,
      space: ColorSpace::RGB,
    }
  }
}

// TODO more here for color warping
