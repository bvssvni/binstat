use BinGraph;

/// Contains various symmetries that compactifies the representation.
pub enum Symmetry<TAction> {
    /// No new bits introduces, just a way to refer to the graph itself.
    NoSymmetry(BinGraph<TAction>),
    /// N new bits that can take on values 100, 010, 001 etc.
    /// The sub symmetries takes up the same space.
    /// This is used when you want define separate sets of rules
    /// that operate in the same context.
    ExclusiveSymmetry(Vec<Symmetry<TAction>>),
    /// Sub symmetries are laid out sequentially.
    /// This is used when you want to combine sets of rules
    /// that are completely independent.
    IndependentSymmetry(Vec<Symmetry<TAction>>),
}


