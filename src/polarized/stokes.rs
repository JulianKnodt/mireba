use quick_maths::{Vec4, Vector};

/// Represents a stokes vector
pub type Stokes = Vec4;

pub const fn unpolarized() -> Stokes { Vector([1., 0., 0., 0.]) }
