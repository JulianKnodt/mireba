use super::Sampler;
use quick_maths::{DefaultFloat, Vector};
use rand::{distributions::Distribution, rngs::SmallRng, Rng, SeedableRng};
use rand_distr::Normal;

#[derive(Debug)]
pub struct Metropolis<const N: usize> {
  rng: SmallRng,
  // jumping distribution
  float_prev: DefaultFloat,
  previous: Vector<DefaultFloat, N>,
}

impl<const N: usize> Sampler for Metropolis<N> {
  fn new(seed: u64) -> Self {
    let mut rng = SmallRng::seed_from_u64(seed);
    Self {
      float_prev: rng.gen(),
      previous: Vector::with(|_| rng.gen()),
      rng,
    }
  }
  /// Just generates a random float for now
  fn sample(&mut self) -> DefaultFloat { self.rng.gen() }
  // TODO
  fn sample_vec<const M: usize>(&mut self) -> Vector<DefaultFloat, M> {
    Vector::with(|i| {
      Normal::new(self.previous[i], 1.0)
        .unwrap()
        .sample(&mut self.rng)
    })
  }
}
