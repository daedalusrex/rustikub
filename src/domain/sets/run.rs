use std::collections::{HashSet, LinkedList};
use crate::domain::tiles::{Number, Tile, Color, ColoredNumber};
use std::vec;
use crate::domain::tiles::Tile::{JokersWild, RegularTile};
use super::ParseError::*;
use super::ParseError;

const MAX_RUN_SIZE: usize = 13;
const MIN_RUN_SIZE: usize = 3;
const MAX_JOKERS_IN_RUN: usize = 2;

/// A set of three or more consecutive numbers all in the same color.
/// The number 1 is always played as the lowest number, it cannot follow the number 13.
#[derive(Debug, PartialEq)]
pub struct Run {
    // Idea here is to decompose what defines a run, and not be dependent on implementation details of std containers
    start: Number,
    end: Number,
    color: Color,
    jokers: HashSet<Number>,
    // list: Vec<Number>, // TODO Maybe later
}

impl Run {
    // Using the Result<T, E> type instead of Option here. It's better suited for this? than Option
    //  https://doc.rust-lang.org/rust-by-example/error/result.html
    pub fn parse(candidates: Vec<Tile>) -> Result<Run, ParseError> {
        if candidates.len() < MIN_RUN_SIZE {
            return Err(TooFewTiles);
        }
        if candidates.len() > MAX_RUN_SIZE {
            return Err(TooManyTiles);
        }

        let mut jokers: HashSet<Number> = HashSet::new();
        let mut end = Number::One;
        let mut start = Number::Thirteen;
        let mut color = Color::Red;
        let mut prepend_joker_count = 0;
        let mut first_defintive_cn: Option<ColoredNumber> = None;
        let mut previous_cn = ColoredNumber::new(color, Number::Thirteen);


        // TODO Alternate approach, acquire the index, and do prepend based on that -> hmm
        // candidates.iter().enumerate().filter()
        // let mut iter_of_only_regular = candidates.iter().filter(|tile| !tile.is_joker());
        // let first_no_joke = iter_of_only_regular.next().ok_or(IllegalJokers);

        // closure logic checks
        let validate_sequence_with_next = |next: &Tile, prev: ColoredNumber| -> Result<Number, ParseError> {
            if prev.num == Number::Thirteen {
                return Err(OutOfBounds);
            }
            if next.is_joker() && prev.num < Number::Thirteen {
                return Ok(prev.num.next());
            }
            if next.is_number(prev.num) {
                return Err(DuplicateNumbers);
            }
            if !next.is_number(prev.num.next()) {
                return Err(OutOfOrder);
            }
            if !next.is_color(prev.color) {
                return Err(DistinctColors);
            }
            Ok(prev.num.next())
        };

        // Go through the vector and check the constraints
        for (idx, tile) in candidates.iter().enumerate() {
            match tile {
                JokersWild => {
                    match first_defintive_cn {
                        Some(first_cn) => {
                            let next = validate_sequence_with_next(tile, previous_cn)?;
                            jokers.insert(next);
                            end = next;
                            previous_cn = ColoredNumber::new(first_cn.color, next)
                        }
                        None => prepend_joker_count += 1,
                    }
                }
                RegularTile(tile_cn) => {
                    match first_defintive_cn {
                        Some(first_cn) => {
                            let next = validate_sequence_with_next(tile, previous_cn)?;
                            end = next; // Keep incrementing the newly found end representation
                            previous_cn = tile_cn.clone();
                        }
                        None => {
                            // Only happens once, when we first encounter a regular tile
                            first_defintive_cn = Some(tile_cn.clone());
                            start = tile_cn.num;
                            color = tile_cn.color;
                            previous_cn = tile_cn.clone(); // Is this a move or a copy?
                        }
                    }
                }
            }
        }

        for i in 0..prepend_joker_count {
            if start == Number::One {
                return Err(IllegalJokers);
            }

            start = start.prev();
            jokers.insert(start);
        }

        if jokers.len() > MAX_JOKERS_IN_RUN {
            return Err(IllegalJokers);
        }

        Ok(Run {
            start,
            end,
            color,
            jokers,
        })
    }

    pub fn decompose(&self) -> Vec<Tile> {
        let mut current = self.start;
        let mut tiles: Vec<Tile> = vec![];
        while current <= self.end {
            if self.jokers.contains(&current) {
                tiles.push(JokersWild);
            } else {
                let cn = ColoredNumber::new(self.color, current);
                tiles.push(RegularTile(cn))
            }
            current = current.next();
        }
        tiles
    }

    pub fn contains(&self, n: Number) -> bool { self.start <= n && self.end >= n }

    pub fn get_run_color(&self) -> Color { self.color }
}

#[cfg(test)]
mod run_parsing {
    use super::*;
    use crate::domain::tiles::Number::*;
    use crate::domain::sets::ParseError::*;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use crate::domain::tiles::{Color, ColoredNumber, Number};
    use strum::IntoEnumIterator;

    pub fn object_mother_good_run_of_three() -> Vec<Tile> {
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
        let first_tile = happy.first().unwrap().clone();
        let first_cn: ColoredNumber = match first_tile {
            RegularTile(cn) => cn,
            _ => { panic!("Test Broken!") }
        };
        let result = Run::parse(happy);
        assert!(result.is_ok());
        let success = result.unwrap();
        assert_eq!(success.start, first_cn.num);
        assert_eq!(success.color, first_cn.color);
        // TODO more precise success metrics
    }

    #[test]
    fn proper_joker_handling() {
        let first = ColoredNumber::new(Color::get_rand(), Three);
        let second = first.next().unwrap();
        let third = second.next().unwrap();
        let okay1 = vec![RegularTile(first), RegularTile(second), RegularTile(third), JokersWild];
        assert!(Run::parse(okay1.clone()).is_ok());
        let okay2 = vec![JokersWild, RegularTile(first), RegularTile(second), RegularTile(third)];
        assert!(Run::parse(okay2.clone()).is_ok());
        assert_eq!(okay2, Run::parse(okay2.clone()).unwrap().decompose());
        let okay3 = vec![JokersWild, JokersWild, RegularTile(first), RegularTile(second), RegularTile(third)];
        assert!(Run::parse(okay3.clone()).is_ok());
        assert_eq!(okay3, Run::parse(okay3.clone()).unwrap().decompose());
        let okay4 = vec![RegularTile(first), RegularTile(second), RegularTile(third), JokersWild, JokersWild];
        assert!(Run::parse(okay4.clone()).is_ok());
        assert_eq!(okay4, Run::parse(okay4.clone()).unwrap().decompose());
    }

    #[test]
    fn too_many_jokers_at_the_end() {
        let first = ColoredNumber::new(Color::get_rand(), Three);
        let second = first.next().unwrap();
        let third = second.next().unwrap();
        let too_many = vec![RegularTile(first),
                            RegularTile(second),
                            RegularTile(third),
                            JokersWild,
                            JokersWild,
                            JokersWild];
        assert!(Run::parse(too_many.clone()).is_err());
    }


    #[test]
    fn reject_bad_joker_edges_of_run() {
        let first = ColoredNumber::new(Color::get_rand(), Eleven);
        let second = first.next().unwrap();
        let third = second.next().unwrap();
        let success = vec![RegularTile(first), RegularTile(second), RegularTile(third), JokersWild];
        assert!(Run::parse(success.clone()).is_err());

        let first = ColoredNumber::new(Color::get_rand(), One);
        let second = first.next().unwrap();
        let third = second.next().unwrap();
        let success = vec![JokersWild, RegularTile(first), RegularTile(second), RegularTile(third)];
        assert!(Run::parse(success.clone()).is_err());
    }

    #[test]
    fn parse_failure_cases_quantity() {
        let happy = object_mother_good_run_of_three();
        let first_tile = happy.first().unwrap().clone();
        let first_cn: ColoredNumber = match first_tile {
            RegularTile(cn) => cn,
            _ => { panic!("Test Broken!") }
        };
        //too few
        let mut too_few = happy.clone();
        too_few.pop();
        assert!(Run::parse(too_few.clone()).is_err());
        // Can also Specify Error Type
        assert_eq!(Err(TooFewTiles), Run::parse(too_few));

        //too many
        let mut too_many = happy.clone();
        for num in Number::iter() {
            too_many.push(RegularTile(ColoredNumber { color: first_cn.color, num }))
        }
        assert!(Run::parse(too_many).is_err());
    }

    #[test]
    fn parse_one_distinct_color()
    {
        let mut distinct_colors = object_mother_good_run_of_three();
        let first_tile = distinct_colors.first().unwrap().clone();
        let first_cn: ColoredNumber = match first_tile {
            RegularTile(cn) => cn,
            _ => { panic!("Test Broken!") }
        };
        distinct_colors.push(RegularTile(ColoredNumber::new(first_cn.color.next(), first_cn.num.prev())));
        assert!(Run::parse(distinct_colors).is_err());
    }

    #[test]
    fn rejects_duplicate_number()
    {
        let mut dupped = object_mother_good_run_of_three();
        let first_tile = dupped.first().unwrap().clone();
        let first_cn: ColoredNumber = match first_tile {
            RegularTile(cn) => cn,
            _ => { panic!("Test Broken!") }
        };
        dupped.push(RegularTile(first_cn.clone()));
        assert!(Run::parse(dupped).is_err());
    }

    #[test]
    fn rejects_reversed_ordering()
    {
        let mut reversed = object_mother_good_run_of_three();
        reversed.reverse();
        assert!(Run::parse(reversed).is_err());
    }

    #[test]
    fn rejects_1_after_13()
    {
        let first = ColoredNumber::new(Color::get_rand(), Twelve);
        let second = ColoredNumber::new(first.color, Thirteen);
        let third = ColoredNumber::new(first.color, One);
        let end_at_13: Vec<Tile> = vec![RegularTile(first), RegularTile(second), RegularTile(third)];
        assert!(Run::parse(end_at_13).is_err());
    }

    /// Could accidentally pass sometimes if random ordering is actually correct
    /// shrugs whatever good enough
    #[test]
    fn rejects_out_of_order_random_numbers()
    {
        let first = ColoredNumber::new(Color::get_rand(), Number::get_rand());
        let second = ColoredNumber::new(first.color, Number::get_rand());
        let third = ColoredNumber::new(first.color, Number::get_rand());
        let random_order: Vec<Tile> = vec![RegularTile(first), RegularTile(second), RegularTile(third)];
        assert!(Run::parse(random_order).is_err());
    }
}


#[cfg(test)]
mod other_tests_of_runs {
    use super::*;
    use crate::domain::tiles::Number::*;
    use crate::domain::sets::ParseError::*;
    use crate::domain::tiles::Tile::RegularTile;
    use crate::domain::tiles::{Color, ColoredNumber, Number};
    use strum::IntoEnumIterator;

    fn good_run() -> (Vec<Tile>, Run) {
        let original = run_parsing::object_mother_good_run_of_three();
        let run = Run::parse(original.clone()).unwrap();
        (original, run)
    }


    #[test]
    fn decomposition_matches() {
        let (origin, run) = good_run();
        assert_eq!(origin, run.decompose())
    }

    #[test]
    fn contains_correct() {
        let (origin, run) = good_run();
        for item in origin {
            if let RegularTile(cn) = item {
                assert!(run.contains(cn.num))
            } else { panic!("Test Broken!") }
        }
    }
}