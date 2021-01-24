use crate::interaction::MediumInteraction;
use quick_maths::Ray3;

use std::fmt::Debug;

pub type Phase = ();

pub trait Medium: Debug {
  fn phase(&self) -> &Phase;
  fn scattering_coeff(&self) -> f32;
  fn absorbtion_coeff(&self) -> f32;
  fn sample(&self, r: &Ray3) -> MediumInteraction;
}
