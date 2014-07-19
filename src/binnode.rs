use std::collections::Bitv;
use BinGraph;

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
            .filter(|&(_, v)| v)
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
        self.with_choices(|i| if !graph.contains_choice(self, i) {
            f(i) 
        })
    }
}

