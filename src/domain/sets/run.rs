use std::collections::{HashSet, LinkedList};
use crate::domain::tiles::{Number, Tile, Color, ColoredNumber};
use std::vec;
use super::ParseError::*;
use super::ParseError;

const MAX_RUN_SIZE: usize = 13;
const MIN_RUN_SIZE: usize = 3;

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
    /// Using the Result<T, E> type instead of Option here. It's better suited for this? than Option
    //  https://doc.rust-lang.org/rust-by-example/error/result.html
    pub fn parse(candidates: Vec<Tile>) -> Result<Run, ParseError> {
        if candidates.len() < MIN_RUN_SIZE {
            return Err(TooFewTiles)
        }
        if candidates.len() > MAX_RUN_SIZE {
            return Err(TooManyTiles)
        }


        let mut iter_of_only_regular = candidates.iter().filter(|tile| !tile.is_joker());
        let first_no_joke = iter_of_only_regular.next().ok_or(IllegalJokers);
        let first_cn: ColoredNumber;
        if let Tile::RegularTile(cn) = first_no_joke.unwrap() {
            first_cn = cn.clone();
        } else {
            return Err(IllegalJokers);
        }

        let mut jokers: HashSet<Number> = HashSet::new();
        let looped_cn: ColoredNumber;
        let mut found_first_cn = false;

        // Go through the vector and check the constraints
        for (idx, tile) in candidates.iter().enumerate() {
            match tile {
                Tile::JokersWild => {
                    // TODO whooopsie, more thinking needed
                }
                Tile::RegularTile(tile_cn) => {
                    if tile_cn.color != first_cn.color {
                        return Err(DistinctColors)
                    }
                }
            }
        }



        Ok(Run{
            start: first_cn.num, // TODO Wrong
            end: first_cn.num, // TODO Wrong
            color: first_cn.color,
            jokers: Default::default()
        })
    }

    pub fn decompose(&self) -> Vec<Tile> {
        vec![] // TODO
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
        let first = ColoredNumber::new(Color::get_rand(), Two);
        let second = first.next().unwrap();
        let third = second.next().unwrap();
        let success = vec![RegularTile(first), RegularTile(second), RegularTile(third), JokersWild];
        assert!(Run::parse(success.clone()).is_ok());

        let mut too_many = success.clone();
        too_many.append( &mut vec![JokersWild, JokersWild]);
        assert!(Run::parse(success.clone()).is_err());
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