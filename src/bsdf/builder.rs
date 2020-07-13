use super::BSDFImpl;
use crate::spectrum::Spectrum;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Builder {
  Diffuse(Spectrum),
  MTL(String),
  Debug,
}

impl From<Builder> for BSDFImpl {
  fn from(b: Builder) -> Self {
    use Builder::*;
    match b {
      Debug => BSDFImpl::Debug(super::debug::Debug),
      Diffuse(s) => BSDFImpl::Diffuse(super::diffuse::Diffuse::new(s)),
      MTL(src) => {
        let mut mtls = vec![];
        let f = std::fs::File::open(src).expect("Failed to open MTL file");
        super::mtl::read_mtl(f, &mut mtls).expect("Failed to read MTL file");
        if mtls.len() > 1 {
          println!("Currently can only handle 1 material per MTL file but multiple specified");
        }
        BSDFImpl::MTL(mtls.remove(0))
      },
    }
  }
}
