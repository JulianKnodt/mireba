#![allow(incomplete_features)]
#![feature(const_generics)]

// Linear algebra modules
pub mod map;
pub mod mat;
pub mod vec;

pub mod bounds;
// pub mod octree2;

// #[macro_use]
// pub mod timing;

pub mod aabox;
pub mod color;
pub mod dcel;
pub mod indexed_triangles;
pub mod material;
pub mod octree;
pub mod plane;
pub mod renderable;
pub mod screen;
pub mod sphere;
pub mod util;
pub mod vis;

// 2D drawing utilities
pub mod lgram;
pub mod polygon;
pub mod turtle;
pub mod spline;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
/// Property checking for various components
#[cfg(test)]
mod properties;
/// Boilerplate code for setting up quicker
#[cfg(test)]
mod testing;
