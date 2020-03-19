/*
A simple L-grammar module for constructing fractals which should be used in generation
*/

#[derive(Debug, PartialEq, Default, Clone)]
pub struct LGrammar<V: Copy, P>
where
  P: (Fn(V) -> Vec<V>) + Clone, {
  /// Current alphabet set for this LGrammar
  axiom: Vec<V>,
  production_rules: P,
}

impl<V: Copy, P> From<(V, P)> for LGrammar<V, P>
where
  P: Fn(V) -> Vec<V> + Clone,
{
  fn from(init: (V, P)) -> Self {
    let (v, p) = init;
    LGrammar {
      axiom: vec![v],
      production_rules: p,
    }
  }
}

impl<V: Copy, P> LGrammar<V, P>
where
  P: Fn(V) -> Vec<V> + Clone,
{
  pub fn next(&self) -> Self {
    let successor = self
      .axiom
      .iter()
      .flat_map(|v| (self.production_rules)(*v))
      .collect::<Vec<_>>();
    LGrammar {
      axiom: successor,
      production_rules: self.production_rules.clone(),
    }
  }
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
}

#[cfg(test)]
mod test {
  use super::LGrammar;
  /// Represents a two state alphabet
  #[derive(Copy, Clone, Debug, PartialEq, Eq)]
  enum BiState {
    A,
    B,
  }

  fn algae(v: BiState) -> Vec<BiState> {
    match v {
      BiState::A => vec![BiState::A, BiState::B],
      BiState::B => vec![BiState::A],
    }
  }

  #[test]
  fn algae_test() {
    let lg = LGrammar::from((BiState::A, algae));
    lg.next();
  }
}
