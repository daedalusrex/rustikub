use super::tiles::Tile;
use std::collections::HashSet;

enum Set {
    // There are two kinds of sets, either a group or a run
    Group,
    Run,
}
struct Group {
    //A set of either three or four tiles of the same number in different colors.
    members: HashSet<Tile>,
}
impl Group {
    fn new(candidates: Vec<Tile>) -> Option<Group> {
        // TODO something like parsing with the new operator here to enforce characteristics
        let mut foo = HashSet::new();
        Option::Some(Group { members: foo })
    }
}
struct Run {
    // a set of three or more consecutive numbers all in the same color.
    // The number 1 is always played as the lowest number, it cannot follow the number 13.
    members: Vec<Tile>, // TODO, Consider a LinkedList
}
struct Table {
    // Publicly viewable and mutable by all players, has all the sets that have been placed
    sets: Vec<Set>,
}
