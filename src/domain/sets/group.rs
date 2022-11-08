use crate::domain::tiles::{Color, Number, Tile};
use std::collections::{HashMap, HashSet};

const MAX_GROUP_SIZE: usize = 4;
const MIN_GROUP_SIZE: usize = 3;

#[derive(Debug, PartialEq)]
pub struct Group {
    //A set of either three or four tiles of the same number in different colors.
    num: Number,
    colors: HashSet<Color>,
    jokers: u8,
}

impl Group {
    fn parse(candidates: Vec<Tile>) -> Option<Group> {
        if candidates.len() > MAX_GROUP_SIZE || candidates.len() < MIN_GROUP_SIZE {
            return None;
        }

        let mut group_number: Number = Number::One;
        let mut num_jokers: u8 = 0;
        let mut cols = HashSet::new();
        // Find the first regular tile, that has a number
        for tile in candidates {
            match tile {
                Tile::JokersWild => num_jokers += 1,
                Tile::RegularTile(cn) => {
                    group_number = cn.num;
                    cols.insert(cn.color);
                }
            }
        }
        if num_jokers > 2 {
            return None;
        }
        Some(Group{num: group_number, colors: cols, jokers: num_jokers })
    }

    fn count(&self) -> u8 {
        self.colors.len() as u8
    }

    fn contains(&self, c: Color) -> bool {
        self.colors.contains(&c)
    }
}

#[cfg(test)]
pub mod group_tests {
    use super::*;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use crate::domain::tiles::{Color, ColoredNumber, Number, Tile};
    use std::collections::{HashMap, HashSet};
    use std::vec;

    fn object_mother_good_group_of_three() -> Vec<Tile> {
        let first = ColoredNumber {
            color: Color::get_rand(),
            num: Number::get_rand(),
        };
        let second = ColoredNumber {
            color: first.color.next(),
            num: first.num,
        };
        let third = ColoredNumber {
            color: second.color.next(),
            num: first.num,
        };
        vec![RegularTile(first), RegularTile(second), RegularTile(third)]
    }

    #[test]
    fn test_parsing_good() {
        let success = object_mother_good_group_of_three();
        assert_ne!(None, Group::parse(success.clone()));
        if let Some(good_group) = Group::parse(success.clone()) {
            assert_eq!(success.len() as u8, good_group.count());
            if let Some(RegularTile(test_data)) = success.first() {
                assert_eq!(test_data.num, good_group.num)
            } else {
                panic!("Test is Broken!!!! There should always be something there")
            }

            //Colors in candidates match colors in Group
            for tile in success {
                if let RegularTile(cn) = tile {
                    assert!(good_group.contains(cn.color));
                } else {
                    panic!("Test Broken! Should be No Jokers Here!")
                }
            }
        }
    }

    ///Test cases:
    /// size constraints
    /// fail different numbers
    /// fail same colors
    #[test]
    fn test_parsing_reject_bad() {
        // TODO
    }

    #[test]
    fn test_jokers_parsing() {
        let mut base = object_mother_good_group_of_three();
        // Good Group with one additional joker should succeed!
        base.push(JokersWild);
        assert_ne!(None, Group::parse(base.clone()));
        // Group of 5 with Two Jokers should fail
        base.push(JokersWild);
        assert_eq!(None, Group::parse(base.clone()));
        // Two Jokers is legitimate!
        assert_ne!(
            None,
            Group::parse(vec![Tile::any_regular(), JokersWild, JokersWild])
        );
        // Three Jokers is not
        assert_eq!(None, Group::parse(vec![JokersWild, JokersWild, JokersWild]));
        // One Joker Should Succeed
        let mut just_one_joker = object_mother_good_group_of_three();
        just_one_joker.pop();
        just_one_joker.push(JokersWild);
        assert_ne!(None, Group::parse(just_one_joker));
    }
}
