use std::collections::HashMap;
use super::{lgram::LGrammar, turtle::Builder as TurtleBuilder, Consumer, Producer};
use crate::film::{Builder as FilmBuilder, Film};
use quick_maths::Vec2;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum UVConvention {
  NegOneToOne,
  ZeroToOne,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RawScene {
  uv: UVConvention,
  /// Where will this scene be written to
  film: FilmBuilder,

  /// LGramBuilder
  lgram: LGrammar,

  turtle: TurtleBuilder,
  // Links input sources to drawing tools
  // linker: HashMap<String, String>,
}

impl From<RawScene> for Scene {
  fn from(rs: RawScene) -> Self {
    let RawScene {
      uv,
      film,
      mut lgram,
      turtle,
    } = rs;
    for _ in 0..10 {
      lgram.next();
    }
    Self {
      uv,
      film: film.into(),
      prod: Producer::LGrammar(lgram),
      consumer: Consumer::Turtle(turtle.into()),
    }
  }
}


#[derive(Debug)]
pub struct Scene {
  uv: UVConvention,
  pub film: Film,

  prod: Producer,
  consumer: Consumer,
}

impl Scene {
  pub fn render(&mut self) { self.consumer.consume(&self.prod, &mut self.film); }
}

impl RawScene {
  pub fn example() -> Self {
    RawScene {
      uv: UVConvention::ZeroToOne,
      film: FilmBuilder{
        size: (512, 512),
      },
      lgram: LGrammar{
        axiom: vec![0, 1],
        rules: {
          let mut rules = HashMap::new();
          rules.insert(0, vec![1, 0]);
          rules.insert(1, vec![0, 1]);
          rules
        },
      },
      turtle: TurtleBuilder {
        pos: Vec2::new(0.5, 0.5),
        dir: Vec2::new(0., 1.0),
        rules: {
          let mut rules = HashMap::new();
          use super::turtle::TurtleInstruction;
          rules.insert(0, TurtleInstruction::Move(0.001));
          rules.insert(1, TurtleInstruction::Rotate(3.0));
          rules
        },
      }
    }
  }
}
