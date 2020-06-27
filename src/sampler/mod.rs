pub mod uniform;
// TODO this doesn't work well yet but...
// pub mod metropolis;
pub mod builder;

use quick_maths::{DefaultFloat, Vector};
use std::fmt::Debug;

pub trait Sampler: Debug {
  /// Creates a new instance with a given seed
  fn new(seed: u64) -> Self;
  fn sample(&mut self) -> DefaultFloat;
  fn sample_vec<const N: usize>(&mut self) -> Vector<DefaultFloat, N>;
  // fn sample_spectrum(&mut self) -> Spectrum
}

#[derive(Debug)]
pub enum Samplers {
  Uniform(uniform::Uniform),
}
