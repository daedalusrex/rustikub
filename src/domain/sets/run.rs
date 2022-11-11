use std::collections::LinkedList;
use crate::domain::tiles::Tile;
use std::vec;
use super::ParseError;

const MAX_RUN_SIZE: usize = 13;
const MIN_RUN_SIZE: usize = 3;

#[derive(Debug, PartialEq)]
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
    use crate::domain::tiles::Number::*;
    use crate::domain::tiles::Tile::RegularTile;
    use crate::domain::tiles::{Color, ColoredNumber, Number};

    fn object_mother_good_run_of_three() -> Vec<Tile> {
        let mut first = ColoredNumber::get_rand();
        // Questionable
        if first.num > Eleven {
            first.num = Eleven
        }
        let second = ColoredNumber::new(first.color, first.num.next());
        let third = ColoredNumber::new(first.color, second.num.next());
        vec![RegularTile(first), RegularTile(second), RegularTile(third)]
    }

    #[test]
    fn parse_happy_path() {
        let happy = object_mother_good_run_of_three();
        assert_ne!(None, Run::parse(happy.clone()));
    }

    #[test]
    fn parse_failure_cases() {
        let happy = object_mother_good_run_of_three();
    }
}
