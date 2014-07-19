#![crate_name = "binstat"]
#![deny(missing_doc)]

//! A library for modelling and reasoning about multidimensional
//! binary states.

#[cfg(test)]
extern crate debug;

use std::collections::Bitv;

/// Contains a binary state and the choices.
#[deriving(PartialEq, Eq, Show, Clone)]
pub struct BinNode {
    /// The state composed of bits.
    pub state: Bitv,
    /// The choices represented as bits that can be flipped.
    pub choices: Bitv
}

impl BinNode {
    /// Gets pairs from pair of booleans.
    pub fn from_pairs(pairs: &[(bool, bool)]) -> BinNode {
        BinNode {
            state: pairs.iter().map(|&(a, _)| a).collect(),
            choices: pairs.iter().map(|&(_, b)| b).collect()
        }
    }

    /// Call closure for each available choice.
    #[inline(always)]
    pub fn with_choices(&self, f: |i: uint|) {
        for i in range(0, self.choices.len())
            .zip(self.choices.iter())
            .filter(|&(_, v)| v == true)
            .map(|(i, _)| i
        ) {
            f(i)
        }
    }

    /// Calls closure for all choices that are not in graph.
    #[inline(always)]
    pub fn with_choices_not_in<TAction>(
        &self, 
        graph: &BinGraph<TAction>, 
        f: |i: uint|
    ) {
        self.with_choices(|i| if !graph.contains_choice(self, i) { f(i) })
    }
}

/// Models a multidimensional binary state.
pub struct BinGraph<TAction> {
    /// The states and the bits that can be flipped.
    pub data: Vec<BinNode>,
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
        self.data.push(BinNode::from_pairs(pairs));
    }

    /// Returns true if the graph contains a choice.
    pub fn contains_choice(&self, state: &BinNode, i: uint) -> bool {
        self.data.iter().any(|st|
            range(0, st.choices.len()).all(|j|
                state.choices.get(j) == if j == i {
                        !state.choices.get(j)
                    } else {
                        state.choices.get(j)
                    }
            )
        )
    }

    /// Returns the suggestion in default state that are open to all choices.
    pub fn default_suggestion(&self) -> BinNode {
        let open_for_all_choices: Vec<(bool, bool)> =
            range(0, self.actions.len()).map(|_| (false, true)).collect();
        BinNode::from_pairs(open_for_all_choices.as_slice())
    }

    /// Suggests a missing state with choices.
    pub fn suggestion(&self) -> Option<BinNode> {
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
    assert_eq!(suggestion, BinNode::from_pairs([
        (false, true),
        (false, true)
    ]));

    g.data.push(suggestion.clone());

    assert!(!g.contains_choice(&suggestion, 0));

    g.push_pairs([
        (true, true), // is swimming, can stop.
        (false, false), // is not reading book, can't read book.
    ]); 
}


