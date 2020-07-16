pub mod identity;

use crate::film::Film;

/// Reconstruction filter trait
pub trait RFilter: Debug {
  fn reconstruct(&self, film: &mut Film);
}
