#![allow(incomplete_features)]
#![feature(const_generics, const_generic_impls_guard)]

pub mod accelerator;
pub mod bounds;
pub mod bsdf;
pub mod camera;
pub mod film;
pub mod integrator;
pub mod interaction;
pub mod light;
pub mod sampler;
pub mod scene;
pub mod shapes;
pub mod spectrum;

pub mod utils;

pub mod transform;

// TODO create cfg for just 2d
pub mod two_d;

/*
// general utility module
pub mod util;

// Convenience trait to make all floats also debug
pub mod num;

// Rendering structures
pub mod brdf;
pub mod object;
pub mod scene;
pub mod vis;

// Serialization types
pub mod indexed_triangles;
pub mod mtl;

// Bounding structures
pub mod bounds;

// Shapes and geometry
pub mod aabox;
pub mod camera;
pub mod color;
pub mod dcel;
pub mod light;
pub mod material;
pub mod plane;
pub mod renderable;
pub mod screen;
pub mod sphere;
pub mod transform;
pub mod triangle;

// 2D drawing utilities
pub mod lgram;
pub mod polygon;
pub mod spline;
pub mod turtle;

*/
#[cfg(test)]
extern crate quickcheck;
/*
/// Property checking for various components
#[cfg(test)]
mod properties;
*/
#[cfg(test)]
mod unit_tests;
