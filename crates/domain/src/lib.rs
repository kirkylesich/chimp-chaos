#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! Domain layer: models and ports (interfaces) with no infrastructure dependencies.

pub mod models;
pub mod ports;
pub mod errors;

#[cfg(test)]
mod tests;

pub mod tests_support;

