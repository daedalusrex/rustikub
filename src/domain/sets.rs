use super::tiles::Tile;
use std::collections::HashSet;
use std::vec;
use std::vec::IntoIter;

pub enum Set {
    // There are two kinds of sets, either a group or a run
    Group(Group),
    Run(Run),
}
pub struct Group {
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

pub struct Run {
    // a set of three or more consecutive numbers all in the same color.
    // The number 1 is always played as the lowest number, it cannot follow the number 13.
    members: Vec<Tile>, // TODO, Consider a LinkedList
}

impl Run {
    fn parse(candidates: Vec<Tile>) -> Option<Run> {
        // for tile in candidates {
        //     todo!()
        // }
        Option::Some(Run {
            members: candidates.clone(),
        })
    }
}

impl IntoIterator for Run {
    type Item = Tile;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}

#[cfg(test)]
mod run_tests {
    use super::super::tiles::{Color, ColoredNumber, Number};
    use super::*;

    #[test]
    fn run_equality() {
        // Vectors have equality comparison by default
        let vec1 = vec![1, 2, 3, 4];
        let vec2 = vec![1, 2, 3, 4];
        let vec_diff = vec![5, 4, 3, 2, 1];
        let vec_ord = vec![3, 4, 2, 1];
        assert_eq!(vec1, vec2);
        assert_ne!(vec1, vec_diff);
        assert_ne!(vec2, vec_ord);

        // let tile_vec = vec![];
    }
}
