use BinNode;

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
        self.data.iter().any(|n| {
            range(0, n.state.len()).all(|j| {
                let state_val = state.state.get(j);
                let state_val = if i == j { !state_val } else { state_val };
                n.state.get(j) == state_val
            })
        })
    }

    /// Finds out if graph is complete.
    pub fn is_complete(&self) -> bool {
        let mut complete = true;
        for node in self.data.iter() {
            node.with_choices_not_in(self, |_| complete = false);
            if !complete { return false }
        }

        true
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
        (true, true), // can stop swimming.
        (false, false), // can't read book.
    ]); 

    assert!(g.contains_choice(&suggestion, 0));
    assert!(g.contains_choice(&g.data[1], 0));

    assert!(!g.is_complete());

    g.push_pairs([
        (false, false), // can't start swimming.
        (true, true), // can stop reading book.
    ]);

    assert!(g.is_complete()); 
}


