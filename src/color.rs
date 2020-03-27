use crate::vec::Vec3;
use num::Float;

/// Tuple of color and alpha value
#[derive(Debug, Clone, Copy)]
pub struct Color<D: Float, C>(C, D);

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct RGB<D = f32>(Vec3<D>);

impl<D: Float> RGB<D> {
  pub fn new(r: D, g: D, b: D) -> Self { RGB(Vec3(r, g, b)) }
  pub fn gamma(&self, gamma: D) -> Self { RGB(self.0.apply_fn(|i| i.powf(gamma))) }
  pub fn tone(t: D) -> Self {
    let t = num::clamp(t, D::zero(), D::one());
    RGB(Vec3::from(t))
  }
  pub fn to_hsl(&self) -> HSL<D> {
    let Vec3(r, g, b) = self.0;
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
    HSL(hsl)
  }
  /// Returns a value which is scaled to the [0, 255] range.
  pub fn val(&self) -> Vec3<D> {
    let Vec3(i, j, k) = self.0;
    assert!(i <= D::one() && j <= D::one() && k <= D::one());
    assert!(i >= D::zero() && j >= D::zero() && k >= D::zero());
    let max = D::from(255.0).unwrap();
    Vec3(i * max, j * max, k * max)
  }
}

impl<D: Float> From<Vec3<D>> for RGB<D> {
  fn from(rgb: Vec3<D>) -> Self { RGB(rgb) }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct HSL<D = f32>(Vec3<D>);

// TODO more here for color warping
