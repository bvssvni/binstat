#![crate_name = "binstat"]
#![deny(missing_doc)]

//! A library for modelling and reasoning about multidimensional
//! binary states.

#[cfg(test)]
extern crate debug;

use std::collections::Bitv;

/// Contains a binary state and the choices.
#[deriving(PartialEq, Eq, Show)]
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

    /// Returns the suggestion in default state that are open to all choices.
    pub fn default_suggestion(&self) -> BinStateChoices {
        let open_for_all_choices: Vec<(bool, bool)> =
            range(0, self.actions.len()).map(|_| (false, true)).collect();
        BinStateChoices::from_pairs(open_for_all_choices.as_slice())
    }

    /// Suggests a missing state with choices.
    pub fn suggestion(&self) -> Option<BinStateChoices> {
        if self.data.len() == 0 {
            Some(self.default_suggestion())
        } else {
            None
        }
    }
}

#[test]
fn test_exclusive() {
    let mut g: BinGraph<&'static str> = BinGraph::new();
    g.actions.push("Go swiming");
    g.actions.push("Read a book");

    let suggestion = g.suggestion().unwrap();
    assert_eq!(suggestion, BinStateChoices::from_pairs([
        (false, true),
        (false, true)
    ]));

    g.data.push(suggestion);

    g.push_pairs([
        (true, true), // is swimming, can stop.
        (false, false), // is not reading book, can't read book.
    ]); 
}


