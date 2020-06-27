use super::{uniform::Uniform, Sampler, Samplers};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Builder {
  pub seed: u64,
  pub variant: Variant,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Variant {
  Uniform,
}

impl From<Builder> for Samplers {
  fn from(b: Builder) -> Samplers {
    let Builder { seed, variant } = b;
    match variant {
      Variant::Uniform => super::Samplers::Uniform(Uniform::new(seed)),
    }
  }
}
