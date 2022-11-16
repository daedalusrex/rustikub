use super::sets::Set;

#[derive(Debug, Clone)]
pub struct Table {
    // Publicly viewable and mutable by all players, has all the sets that have been placed
    sets: Vec<Set>,
}

// TODO consider renaming table to face up
impl Table {
    /// Rules have several types of manipulations
    /// Add one or more tiles from rack to make new set
    /// Remove a fourth tile from a group and use it to form a new set:
    /// Add a fourth tile to a set and remove one tile from it, to make another set:
    /// Splitting a run
    /// Combined split
    /// Multiple split:
    /// Also, special joker rules need to be considered
    pub fn manipulate() {}

    pub fn new() -> Table {
        //TODO
        Table{sets: vec![]}
    }
}
