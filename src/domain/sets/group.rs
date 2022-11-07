use std::collections::{HashMap, HashSet};
use crate::domain::tiles::{Color, Number, Tile};

const MAX_GROUP_SIZE: usize = 4;
const MIN_GROUP_SIZE: usize = 3;

pub struct Group {
    //A set of either three or four tiles of the same number in different colors.
    num: Number,
    colors: HashSet<Color>,
}

impl Group {
    fn parse(candidates: Vec<Tile>) -> Option<Group> {
        if candidates.len() > MAX_GROUP_SIZE ||  candidates.len() < MIN_GROUP_SIZE {
            return None;
        }
        let mut first_num = Number::One;
        // Find the first regular tile, that has a number
        for tile in candidates {
            if let Tile::RegularTile(syntax) = tile {
                first_num == syntax.num;
                break;
            }
            return None;
        }
        None
    }

    fn count(&self) -> u8 {

        self.colors.len() as u8
    }
}


#[cfg(test)]
pub mod group_tests {
    use std::vec;
    use std::collections::{HashMap, HashSet};
    use crate::domain::tiles::{Color, ColoredNumber, Number, Tile};
    use crate::domain::tiles::Tile::RegularTile;

    fn object_mother_good_group() -> Vec<Tile> {
        let base = ColoredNumber{color: Color::get_rand(), num: Number::get_rand()};
        let base = Tile::any_regular();

        let foo = RegularTile(other);

        // let foo = vec![Tile::any_regular(), Tile::JokersWild];
    }

    ///Test cases:
    /// all jokers
    /// size constraints
    /// two jokers? (I think this is only allowed when size of 4? todo check rules)
    /// fail different numbers
    /// fail same colors
    /// success cases also of course
    #[test]
    fn test_parsing() {




    }
}