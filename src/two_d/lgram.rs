use std::collections::HashMap;
/*
A simple L-grammar module for constructing fractals which should be used in generation
*/

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LGrammar {
  /// Initial axiom
  pub axiom: Vec<u8>,

  pub rules: HashMap<u8, Vec<u8>>,
}

impl LGrammar {
  pub fn next(&mut self) -> &mut Self {
    let rules = &self.rules;
    let successor = self
      .axiom
      .drain(..)
      .flat_map(|v| rules[&v].clone())
      .collect::<Vec<_>>();
    self.axiom = successor;
    self
  }
  /*
  pub fn nth(&self, n: u32) -> Self {
    if n == 0 {
      return (*self).clone();
    }
    // TODO convert this into not using temporary vectors
    // hm not entirely sure how to do this non-trivially
    (0..(n - 1)).fold(self.next(), move |acc, _| acc.next())
  }
  pub fn finalize(mut self) -> Vec<V> {
    self.axiom.shrink_to_fit();
    self.axiom
  }
  */
}
