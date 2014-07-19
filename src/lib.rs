#![crate_name = "binstat"]
#![deny(missing_doc)]

//! A library for modelling and reasoning about multidimensional
//! binary states.

use std::collections::Bitv;

/// Contains a binary state and the choices.
pub struct BinStateChoices {
    /// The state composed of bits.
    pub state: Bitv,
    /// The choices represented as bits that can be flipped.
    pub choices: Bitv
}

impl BinStateChoices {
    /// Gets pairs from pair of booleans.
    pub fn from_pairs(pairs: &[(bool, bool)]) -> BinStateChoices {
        BinStateChoices {
            state: pairs.iter().map(|&(a, _)| a).collect(),
            choices: pairs.iter().map(|&(_, b)| b).collect()
        }
    }
}

/// Models a multidimensional binary state.
pub struct BinGraph<TAction> {
    /// The states and the bits that can be flipped.
    pub data: Vec<BinStateChoices>,
    /// A description the action that takes when flipping bits.
    pub actions: Vec<TAction>,
}

impl<TAction> BinGraph<TAction> {
    /// Creates a new BinGraph.
    pub fn new() -> BinGraph<TAction> {
        BinGraph {
            data: Vec::new(),
            actions: Vec::new()
        }
    }

    /// Adds new information about state and choices.
    ///
    /// The first bool indicate state, the second whether it can change.
    pub fn push_pairs(&mut self, pairs: &[(bool, bool)]) {
        self.data.push(BinStateChoices::from_pairs(pairs));
    }
}

#[test]
fn test_exclusive() {
    let mut g: BinGraph<&'static str> = BinGraph::new();
    g.actions.push("Go swiming");
    g.actions.push("Read a book");

    g.push_pairs([
        (false, true), // go swimming
        (false, true), // read a book
    ]);
    g.push_pairs([
        (true, true), // is swimming, can stop.
        (false, false), // is not reading book, can't read book.
    ]); 
}


