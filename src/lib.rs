#![crate_name = "binstat"]
#![deny(missing_doc)]

//! A library for modelling and reasoning about multidimensional
//! binary states.

#[cfg(test)]
extern crate debug;

pub use binnode::BinNode;
pub use bingraph::BinGraph;
pub use symmetry::Symmetry;

mod binnode;
mod bingraph;
mod symmetry;

