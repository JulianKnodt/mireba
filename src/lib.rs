#![warn(clippy::many_single_char_names)]
#![allow(incomplete_features)]
#![feature(const_generics)]

pub const EPS: f32 = 0.000001;

pub mod accelerator;
pub mod bounds;
pub mod bsdf;
pub mod camera;
pub mod film;
pub mod integrator;
pub mod interaction;
pub mod light;
pub mod medium;
pub mod polarized;
pub mod sampler;
pub mod scene;
pub mod shapes;
pub mod spectrum;
pub mod texture;

pub mod utils;

pub mod transform;

// TODO create cfg for just 2d
pub mod two_d;

#[cfg(test)]
extern crate quickcheck;
/*
/// Property checking for various components
#[cfg(test)]
mod properties;
*/
#[cfg(test)]
mod unit_tests;
