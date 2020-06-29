use super::Texture;
use crate::spectrum::{from_rgb, Spectrum};
use image::{Rgb, RgbImage};
use quick_maths::{Vec2, Vec3, Vector};

#[derive(Debug)]
pub struct Bitmap {
  img: RgbImage,
}

impl Texture for Bitmap {
  fn sample(&self, uv: Vec2) -> Spectrum {
    let uv = uv * Vec2::new(self.img.width() as f32, self.img.height() as f32);
    let Vector([u, v]) = uv.apply_fn(|v| v as u32);
    let Rgb([r, g, b]) = self.img.get_pixel(u, v);
    from_rgb(Vec3::new(r, g, b).apply_fn(|&v| v as f32) / 255.0)
  }
}
