use super::sets::Set;

#[derive(Debug, Clone)]
pub struct Table {
    // Publicly viewable and mutable by all players, has all the sets that have been placed
    sets: Vec<Set>,
}
