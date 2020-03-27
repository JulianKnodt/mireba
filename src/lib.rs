#![allow(incomplete_features)]
#![feature(const_generics)]

// Linear algebra modules
pub mod map;
pub mod mat;
pub mod vec;

// Rendering structures
pub mod object;
pub mod scene;
pub mod vis;

// Bounding structures
pub mod bounds;
// pub mod octree2;
// pub mod octree;

// #[macro_use]
// pub mod timing;

pub mod light;

pub mod aabox;
pub mod color;
pub mod dcel;
pub mod indexed_triangles;
pub mod material;
pub mod plane;
pub mod renderable;
pub mod screen;
pub mod sphere;
pub mod util;

// 2D drawing utilities
pub mod lgram;
pub mod polygon;
pub mod spline;
pub mod turtle;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
/// Property checking for various components
#[cfg(test)]
mod properties;
/// Boilerplate code for setting up quicker
#[cfg(test)]
mod testing;
