/// Scalar value operations

use num::{Float, Zero, One};

fn smooth_step<D: Float + One + Zero>(min: D, max: D, v: D) -> D {
  if v < min {
    D::zero()
  } else if v > max {
    D::one()
  } else {
    v
  }
}

// TODO implement smooth min and smooth max
