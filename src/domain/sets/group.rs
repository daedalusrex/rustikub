use crate::domain::tiles::{Color, ColoredNumber, Number, Tile};
use std::collections::{HashMap, HashSet};

const MAX_GROUP_SIZE: usize = 4;
const MIN_GROUP_SIZE: usize = 3;

///A set of either three or four tiles of the same number in different colors.
#[derive(Debug, PartialEq)]
pub struct Group {
    num: Number,
    colors: HashSet<Color>,
    jokers: u8,
}

impl Group {
    /// Checks the given candidate tiles against a logical constraints that define a Group
    /// If sucessful returns a Group composed of those tiles, otherwise None
    pub fn parse(candidates: Vec<Tile>) -> Option<Group> {
        if candidates.len() > MAX_GROUP_SIZE || candidates.len() < MIN_GROUP_SIZE {
            return None;
        }

        let mut group_number: Number = Number::One;
        let mut num_jokers: u8 = 0;
        let mut cols = HashSet::new();
        let first_num: Number;


        // TODO Simplify with ?: https://doc.rust-lang.org/rust-by-example/error/option_unwrap/question_mark.html
        let first_no_joke = candidates.iter().filter(|tile| !tile.is_joker()).next();
        if let Some(first_tile) = first_no_joke {
            if let Tile::RegularTile(first_cn) = first_tile {
                first_num = first_cn.num;
            } else {
                return None;
            }
        }
        else {
            return None
        }

        // Find the first regular tile, that has a number
        for tile in candidates {
            match tile {
                Tile::JokersWild => num_jokers += 1,
                Tile::RegularTile(cn) => {
                    if first_num != cn.num {
                        return None;
                    }
                    group_number = cn.num;
                    if cols.contains(&cn.color) {
                        return None;
                    }
                    cols.insert(cn.color);
                }
            }
        }
        if num_jokers > 2 {
            return None;
        }
        Some(Group {
            num: group_number,
            colors: cols,
            jokers: num_jokers,
        })
    }

    pub fn count(&self) -> u8 {
        self.colors.len() as u8
    }

    pub fn contains(&self, c: Color) -> bool {
        self.colors.contains(&c)
    }

    pub fn decompose(&self) -> Vec<Tile> {
        let mut composite_tiles: Vec<Tile> = vec![];
        for joker in 0..self.jokers {
            composite_tiles.push(Tile::JokersWild);
        }
        for color in &self.colors {
            composite_tiles.push(Tile::RegularTile(ColoredNumber{
                color: *color,
                num: self.num
            }))
        }
        composite_tiles
    }

    pub fn get_group_num(&self) -> Number { self.num }
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
            for tile in &success {
                if let RegularTile(cn) = tile {
                    assert!(good_group.contains(cn.color));
                } else {
                    panic!("Test Broken! Should be No Jokers Here!")
                }
            }
        }

        let mut with_joker = success.clone();
        with_joker.insert(0, JokersWild);
        let joker_group = Group::parse(with_joker.clone());
        assert_ne!(None, joker_group);

    }

    #[test]
    fn test_parsing_reject_bad() {
        // Size Constraints
        let normal = object_mother_good_group_of_three();
        let mut too_big = normal.clone();
        too_big.append(&mut vec![Tile::any_regular(), Tile::any_regular()]);
        let mut too_small = normal.clone();
        too_small.pop();
        assert_eq!(None, Group::parse(too_big));
        assert_eq!(None, Group::parse(too_small));

        // Different Numbers, Allowable Colors
        let bad_nums = vec![
            RegularTile(ColoredNumber {
                color: Color::Red,
                num: Number::One,
            }),
            RegularTile(ColoredNumber {
                color: Color::Blue,
                num: Number::Two,
            }),
            RegularTile(ColoredNumber {
                color: Color::Black,
                num: Number::Three,
            }),
        ];
        assert_eq!(None, Group::parse(bad_nums));

        // Same Numbers, Duplicate Colors
        let duped_colors = vec![
            RegularTile(ColoredNumber {
                color: Color::Red,
                num: Number::One,
            }),
            RegularTile(ColoredNumber {
                color: Color::Red,
                num: Number::One,
            }),
            RegularTile(ColoredNumber {
                color: Color::Black,
                num: Number::One,
            }),
        ];
        assert_eq!(None, Group::parse(duped_colors));
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

    #[test]
    fn composites_match() {
        let mut origin = object_mother_good_group_of_three();
        origin.sort();
        let my_group = Group::parse(origin.clone()).unwrap();
        let mut output = my_group.decompose();
        output.sort();
        assert_eq!(origin, output);

        let mut with_joker = origin.clone();
        with_joker.push(JokersWild);
        with_joker.sort();
        let joker_group = Group::parse(with_joker.clone()).unwrap();
        let mut joker_out = joker_group.decompose();
        joker_out.sort();
        assert_eq!(with_joker, joker_out);
    }
}
