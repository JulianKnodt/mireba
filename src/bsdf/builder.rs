use super::BSDFImpl;
use crate::spectrum::Spectrum;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Builder {
  Diffuse(Spectrum),
  Debug,
}

impl From<Builder> for BSDFImpl {
  fn from(b: Builder) -> Self {
    use Builder::*;
    match b {
      Debug => BSDFImpl::Debug(super::debug::Debug),
      Diffuse(s) => BSDFImpl::Diffuse(super::diffuse::Diffuse::new(s)),
    }
  }
}
