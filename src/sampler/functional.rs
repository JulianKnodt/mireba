use quick_maths::{Vec2, Vector};

// https://blog.thomaspoulet.fr/uniform-sampling-on-unit-hemisphere/
// Same as below equation with pow = 0
// This returns theta, phi
pub fn square_to_unit_disk(uv: Vec2) -> Vec2 {
  let Vector([u, v]) = uv;
  let theta = (-u * (u - 2.0)).sqrt().asin();
  let phi = 2.0 * std::f32::consts::PI * v;
  Vec2::new(theta, phi)
}

pub fn square_to_cos_power(uv: Vec2, pow: f32) -> Vec2 {
  let Vector([u, v]) = uv;
  let theta = (1.0 - u).powf((pow + 1.0).recip()).acos();
  let phi = 2.0 * std::f32::consts::PI * v;
  Vec2::new(theta, phi)
}

// Linked in Mitsuba so maybe add?
// http://psgraphics.blogspot.com/2011/01/improved-code-for-concentric-map.html
