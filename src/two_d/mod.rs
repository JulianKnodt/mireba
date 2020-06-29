pub mod lgram;
pub mod scene;
pub mod turtle;

use crate::{film::Film, spectrum::from_rgb};
use quick_maths::Vec3;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Producer {
  LGrammar(lgram::LGrammar),
}

impl Producer {
  /// A finite number of items created by this producer
  pub fn items(&self) -> &[u8] {
    match &self {
      Producer::LGrammar(l) => &l.axiom,
    }
  }
}

#[derive(Debug)]
pub enum Consumer {
  Turtle(turtle::Turtle),
}

impl Consumer {
  pub fn consume(&mut self, p: &Producer, film: &mut Film) {
    match self {
      Consumer::Turtle(turt) =>
        for &i in p.items() {
          let curr = turt.curr_pos();
          turt.follow_raw(i);
          let next = turt.curr_pos();
          if curr != next {
            film.line(from_rgb(Vec3::new(0.5, 0.5, 0.5)), curr, next);
          }
        },
    }
  }
}
