use super::RFilter;

#[derive(Debug)]
pub struct Identity;

impl RFilter for Identity {
  // Do nothing in reconstruction
  fn reconstruct(&self, film: &mut Film) {}
}
