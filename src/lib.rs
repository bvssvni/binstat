#![crate_name = "binstat"]
#![deny(missing_doc)]

//! A library for modelling and reasoning about multidimensional
//! binary states.

use std::collections::Bitv;

/// Models a multidimensional binary state.
pub struct BinGraph<TAction> {
    /// The states and the bits that can be flipped.
    pub data: Vec<(Bitv, Bitv)>,
    /// A description the action that takes when flipping bits.
    pub actions: Vec<TAction>,
}

