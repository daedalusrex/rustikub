use super::sets::Set;

struct Table {
    // Publicly viewable and mutable by all players, has all the sets that have been placed
    sets: Vec<Set>,
}
