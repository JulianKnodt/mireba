use super::Sampler;
use quick_maths::{DefaultFloat, Vector};
use rand::{rngs::SmallRng, Rng, SeedableRng};

// https://rust-random.github.io/rand/rand/trait.SeedableRng.html#method.from_entropy
// https://rust-random.github.io/rand/rand/rngs/struct.SmallRng.html
#[derive(Debug)]
pub struct Uniform(SmallRng);

impl Sampler for Uniform {
  fn new(seed: u64) -> Self { Uniform(SmallRng::seed_from_u64(seed)) }
  fn sample(&mut self) -> DefaultFloat { self.0.gen() }
  fn sample_vec<const N: usize>(&mut self) -> Vector<DefaultFloat, N> {
    Vector::with(|_| self.sample())
  }
}
