use std::marker::PhantomData;
use crate::vec::{Vec2, Vector};
/*


/// A cubic spline
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct CubicSpline<D=f32, V=Vec2<D>> {
  /// Control points for use in interpolation
  ctrls: [V; 4],
  _phantom: PhantomData<D>,
}

impl<D, V> CubicSpline<D, V> {
  pub fn new(ctrls: [V; 4]) -> Self {
    Self {
      ctrls,
      _phantom: PhantomData,
    }
  }
}

impl<D: Float, V: Vector<Field=D>> {
  pub fn at(&self, u: D) -> V {
    let u = u.max(D::zero()).min(D::one());
    // also need to add more type constraints to V
  }
}
*/

