use std::collections::LinkedList;
use crate::domain::tiles::Tile;
use std::vec;
use super::ParseError;

pub struct Run {
    // a set of three or more consecutive numbers all in the same color.
    // The number 1 is always played as the lowest number, it cannot follow the number 13.
    members: Vec<Tile>, // TODO, Consider a LinkedList
    
}

impl Run {
    /// Using the Result<T, E> type instead of Option here. It's better suited for this? than Option
    //  https://doc.rust-lang.org/rust-by-example/error/result.html
    fn parse(candidates: Vec<Tile>) -> Result<Run, ParseError> {

        let anon = || { ParseError::OutOfOrder};
        Err(anon())
        //
        // Option::Some(Run {
        //     members: candidates.clone(),
        // })
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
    use super::*;
    use crate::domain::tiles::{Color, ColoredNumber, Number};

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
