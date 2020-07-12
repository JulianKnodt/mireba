use crate::spectrum::{Luminance, RGB};
use quick_maths::{Mat3, Matrix, Vec3, Vector};

/// Color aliases
pub type CIE = Vec3;

// XXX note that all matrices from online must be transposed because we use col major ordering.

/// Matrix for conversion from CIE to sRGB
pub const CIE_TO_SRGB: Mat3 = Matrix(Vector([
  Vector([3.24096994, -0.96924364, 0.05563008]),
  Vector([-1.53738318, 1.8759675, -0.20397696]),
  Vector([-0.49861076, 0.04155506, 1.05697151]),
]));

pub fn cie_to_srgb(cie: &CIE) -> RGB {
  // https://en.wikipedia.org/wiki/SRGB
  CIE_TO_SRGB
    .dot(cie)
    .apply_fn(|u| {
      if u <= 0.0031308 {
        323.0 * u / 25.0
      } else {
        (211. * u.powf(5. / 12.) - 11.) / 200.
      }
    })
    .max(0.)
    .min(1.)
}

pub fn wavelength_to_cie(w: f32) -> CIE {
  // https://en.wikipedia.org/wiki/CIE_1931_color_space
  CIE::new(
    gaussian(w, 1.056, 5998., 379., 310.)
      + gaussian(w, 0.362, 4420., 160., 267.)
      + gaussian(w, -0.065, 5011., 204., 262.),
    gaussian(w, 0.821, 5688., 469., 405.) + gaussian(w, 0.286, 5309., 163., 311.),
    gaussian(w, 1.217, 4370., 118., 360.) + gaussian(w, 0.681, 4590., 260., 138.),
  )
  .min(0.)
  .max(1.)
}

fn gaussian(x: f32, alpha: f32, mu: f32, sigma_1: f32, sigma_2: f32) -> f32 {
  let sqr_rt = (x - mu) / (if x < mu { sigma_1 } else { sigma_2 });
  alpha * (-(sqr_rt * sqr_rt) / 2.).exp()
}

pub fn wavelength_to_srgb(w: f32) -> RGB { cie_to_srgb(&wavelength_to_cie(w)) }

pub fn srgb_to_gray(rgb: RGB) -> Luminance {
  let Vector([r, g, b]) = rgb.apply_fn(|v| {
    if v < 0.0405 {
      v / 12.92
    } else {
      ((v + 0.055) / (1.055)).powf(2.4)
    }
  });
  // https://en.wikipedia.org/wiki/Grayscale
  0.2126 * r + 0.7152 * g + 0.0722 * b
}

/*
impl<D: Float> RGB<D> {
  pub fn new(r: D, g: D, b: D) -> Self { RGB(Vec3(r, g, b)) }
  pub fn red() -> Self { RGB(Vec3::basis(0)) }
  pub fn green() -> Self { RGB(Vec3::basis(1)) }
  pub fn blue() -> Self { RGB(Vec3::basis(2)) }
  pub fn yellow() -> Self { RGB(Vec3(D::one(), D::one(), D::zero())) }
  pub fn purple() -> Self { RGB(Vec3(D::one(), D::zero(), D::one())) }
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
  pub fn into_inner(self) -> Vec3<D> { self.0 }
}

impl<D: Float> From<Vec3<D>> for RGB<D> {
  fn from(rgb: Vec3<D>) -> Self { RGB(rgb) }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct HSL<D = f32>(Vec3<D>);

// TODO more here for color warping
*/
