use super::ParseError;
use super::ParseError::*;
use crate::domain::score_value::ScoreValue;
use crate::domain::tiles::Tile::{JokersWild, RegularTile};
use crate::domain::tiles::{Color, ColoredNumber, Number, Tile};
use crate::domain::Decompose;
use std::collections::{HashSet, LinkedList};
use std::vec;

const MAX_RUN_SIZE: usize = 13;
const MIN_RUN_SIZE: usize = 3;
const MAX_JOKERS_IN_RUN: usize = 2;

/// A set of three or more consecutive numbers all in the same color.
/// The number 1 is always played as the lowest number, it cannot follow the number 13.
#[derive(Debug, PartialEq, Clone)]
pub struct Run {
    // Idea here is to decompose what defines a run, and not be dependent on implementation details of std containers
    start: Number,
    end: Number,
    color: Color,
    jokers: HashSet<Number>,
}

impl Run {
    /// Creates a run based on defining parameters as given in constructor
    pub fn of(start: &ColoredNumber, len: u8) -> Option<Run> {
        if len < MIN_RUN_SIZE as u8 {
            return None;
        }
        let start_val = start.num.as_value();
        if start_val + ScoreValue::of(len) > ScoreValue::of(13) {
            return None;
        }
        let mut end = start.num;
        for _ in 0..len {
            end = end.next();
        }
        Some(Run {
            start: start.num,
            end,
            color: start.color,
            jokers: HashSet::new(),
        })
    }

    // Using the Result<T, E> type instead of Option here. It's better suited for this? than Option
    //  https://doc.rust-lang.org/rust-by-example/error/result.html
    // TODO this should be &Vec<Tile>, a reference instead of a borrow
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
        let validate_sequence_with_next =
            |next: &Tile, prev: ColoredNumber| -> Result<Number, ParseError> {
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
                JokersWild => match first_defintive_cn {
                    Some(first_cn) => {
                        let next = validate_sequence_with_next(tile, previous_cn)?;
                        jokers.insert(next);
                        end = next;
                        previous_cn = ColoredNumber::new(first_cn.color, next)
                    }
                    None => prepend_joker_count += 1,
                },
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

    pub fn contains(&self, n: Number) -> bool {
        self.start <= n && self.end >= n
    }

    pub fn get_run_color(&self) -> Color {
        self.color
    }

    pub fn total_value(&self) -> ScoreValue {
        let mut score_sum = ScoreValue::of(0);
        let mut current = self.start;
        while current <= self.end {
            score_sum = score_sum + current.as_value();
            current = current.next();
        }
        score_sum
    }

    /// takes a candidate tile. If it is possible and allowed to be added returns a NEW run
    /// with the tile attached. Requested Spot is only considered for Jokers, which could be placed
    /// on either end of the run. If none is provided the highest value location will be chosen
    pub fn add_tile(&self, tile: &Tile, requested_spot: Option<Number>) -> Option<Self> {
        // TODO Consider breaking this up into different types of functions, simple ones first, joker later

        // Clojure logic for where to put Joker, only if requested spot is not provided
        let find_highest_target = || -> Option<Number> {
            return if self.end == Number::Thirteen {
                if self.start == Number::One {
                    None // Ridiculous but possible edge case
                } else {
                    Some(self.start.prev())
                }
            } else {
                Some(self.end.next())
            };
        };

        let is_new_location_valid = |num: Option<Number>| -> bool {
            match num {
                None => false,
                Some(num) => {
                    return (num != self.end && num != self.start)
                        && (num == self.end.next() || num == self.start.prev())
                }
            }
        };

        let new_delimiters = |cand: Number| -> (Number, Number) {
            let mut new_start = self.start;
            let mut new_end = self.end;
            if cand > self.end {
                new_end = cand;
            } else if cand < self.start {
                new_start = cand;
            }
            return (new_start, new_end);
        };

        let update_jokers = |num| {
            let mut new_jokers = self.jokers.clone();
            new_jokers.insert(num);
            new_jokers
        };

        match tile {
            JokersWild => {
                if self.jokers.len() >= MAX_JOKERS_IN_RUN {
                    return None;
                } else if is_new_location_valid(requested_spot) {
                    let req = requested_spot.unwrap();
                    let (new_start, new_end) = new_delimiters(req);
                    let new_jokers = update_jokers(req);
                    return Some(Run {
                        start: new_start,
                        end: new_end,
                        jokers: new_jokers,
                        color: self.color,
                    });
                } else if requested_spot.is_none() {
                    let new_spot = find_highest_target();
                    if new_spot.is_none() {
                        return None;
                    }
                    let (new_start, new_end) = new_delimiters(new_spot.unwrap());
                    let new_jokers = update_jokers(new_spot.unwrap());
                    return Some(Run {
                        start: new_start,
                        end: new_end,
                        jokers: new_jokers,
                        color: self.color,
                    });
                }
                return None;
            }
            RegularTile(cn) => {
                if cn.color != self.color || requested_spot.is_some() {
                    return None;
                } else if is_new_location_valid(Some(cn.num)) {
                    let (new_start, new_end) = new_delimiters(cn.num);
                    return Some(Run {
                        start: new_start,
                        end: new_end,
                        color: self.color,
                        jokers: self.jokers.clone(),
                    });
                }
            }
        }
        None
    }
}

impl Decompose for Run {
    fn decompose(&self) -> Vec<Tile> {
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
}

#[cfg(test)]
mod run_parsing {
    use super::*;
    use crate::domain::sets::ParseError::*;
    use crate::domain::tiles::Number::*;
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
            _ => {
                panic!("Test Broken!")
            }
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
        let okay1 = vec![
            RegularTile(first),
            RegularTile(second),
            RegularTile(third),
            JokersWild,
        ];
        assert!(Run::parse(okay1.clone()).is_ok());
        let okay2 = vec![
            JokersWild,
            RegularTile(first),
            RegularTile(second),
            RegularTile(third),
        ];
        assert!(Run::parse(okay2.clone()).is_ok());
        assert_eq!(okay2, Run::parse(okay2.clone()).unwrap().decompose());
        let okay3 = vec![
            JokersWild,
            JokersWild,
            RegularTile(first),
            RegularTile(second),
            RegularTile(third),
        ];
        assert!(Run::parse(okay3.clone()).is_ok());
        assert_eq!(okay3, Run::parse(okay3.clone()).unwrap().decompose());
        let okay4 = vec![
            RegularTile(first),
            RegularTile(second),
            RegularTile(third),
            JokersWild,
            JokersWild,
        ];
        assert!(Run::parse(okay4.clone()).is_ok());
        assert_eq!(okay4, Run::parse(okay4.clone()).unwrap().decompose());
    }

    #[test]
    fn too_many_jokers_at_the_end() {
        let first = ColoredNumber::new(Color::get_rand(), Three);
        let second = first.next().unwrap();
        let third = second.next().unwrap();
        let too_many = vec![
            RegularTile(first),
            RegularTile(second),
            RegularTile(third),
            JokersWild,
            JokersWild,
            JokersWild,
        ];
        assert!(Run::parse(too_many.clone()).is_err());
    }

    #[test]
    fn reject_bad_joker_edges_of_run() {
        let first = ColoredNumber::new(Color::get_rand(), Eleven);
        let second = first.next().unwrap();
        let third = second.next().unwrap();
        let success = vec![
            RegularTile(first),
            RegularTile(second),
            RegularTile(third),
            JokersWild,
        ];
        assert!(Run::parse(success.clone()).is_err());

        let first = ColoredNumber::new(Color::get_rand(), One);
        let second = first.next().unwrap();
        let third = second.next().unwrap();
        let success = vec![
            JokersWild,
            RegularTile(first),
            RegularTile(second),
            RegularTile(third),
        ];
        assert!(Run::parse(success.clone()).is_err());
    }

    #[test]
    fn parse_failure_cases_quantity() {
        let happy = object_mother_good_run_of_three();
        let first_tile = happy.first().unwrap().clone();
        let first_cn: ColoredNumber = match first_tile {
            RegularTile(cn) => cn,
            _ => {
                panic!("Test Broken!")
            }
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
            too_many.push(RegularTile(ColoredNumber {
                color: first_cn.color,
                num,
            }))
        }
        assert!(Run::parse(too_many).is_err());
    }

    #[test]
    fn parse_one_distinct_color() {
        let mut distinct_colors = object_mother_good_run_of_three();
        let first_tile = distinct_colors.first().unwrap().clone();
        let first_cn: ColoredNumber = match first_tile {
            RegularTile(cn) => cn,
            _ => {
                panic!("Test Broken!")
            }
        };
        distinct_colors.push(RegularTile(ColoredNumber::new(
            first_cn.color.next(),
            first_cn.num.prev(),
        )));
        assert!(Run::parse(distinct_colors).is_err());
    }

    #[test]
    fn rejects_duplicate_number() {
        let mut dupped = object_mother_good_run_of_three();
        let first_tile = dupped.first().unwrap().clone();
        let first_cn: ColoredNumber = match first_tile {
            RegularTile(cn) => cn,
            _ => {
                panic!("Test Broken!")
            }
        };
        dupped.push(RegularTile(first_cn.clone()));
        assert!(Run::parse(dupped).is_err());
    }

    #[test]
    fn rejects_reversed_ordering() {
        let mut reversed = object_mother_good_run_of_three();
        reversed.reverse();
        assert!(Run::parse(reversed).is_err());
    }

    #[test]
    fn rejects_1_after_13() {
        let first = ColoredNumber::new(Color::get_rand(), Twelve);
        let second = ColoredNumber::new(first.color, Thirteen);
        let third = ColoredNumber::new(first.color, One);
        let end_at_13: Vec<Tile> =
            vec![RegularTile(first), RegularTile(second), RegularTile(third)];
        assert!(Run::parse(end_at_13).is_err());
    }

    /// Could accidentally pass sometimes if random ordering is actually correct
    /// shrugs whatever good enough
    #[test]
    fn rejects_out_of_order_random_numbers() {
        let first = ColoredNumber::new(Color::get_rand(), Number::get_rand());
        let second = ColoredNumber::new(first.color, Number::get_rand());
        let third = ColoredNumber::new(first.color, Number::get_rand());
        let random_order: Vec<Tile> =
            vec![RegularTile(first), RegularTile(second), RegularTile(third)];
        assert!(Run::parse(random_order).is_err());
    }

    #[test]
    fn score_value_is_correct() {
        let first = ColoredNumber::new(Color::get_rand(), Five);
        let second = ColoredNumber::new(first.color, Six);
        let third = ColoredNumber::new(first.color, Seven);

        let actual_sum = ScoreValue::of(5 + 6 + 7);
        let known_run = Run::parse(vec![
            RegularTile(first),
            RegularTile(second),
            RegularTile(third),
        ])
        .unwrap();
        assert_eq!(known_run.total_value(), known_run.total_value());

        let actual_sum_with_joker = ScoreValue::of(5 + 6 + 7 + 8);
        let with_joker = Run::parse(vec![
            RegularTile(first),
            RegularTile(second),
            RegularTile(third),
            JokersWild,
        ])
        .unwrap();
        assert_eq!(actual_sum_with_joker, with_joker.total_value());
    }
}

#[cfg(test)]
mod other_tests_of_runs {
    use super::*;
    use crate::domain::sets::ParseError::*;
    use crate::domain::tiles::Color::Red;
    use crate::domain::tiles::Number::*;
    use crate::domain::tiles::Tile::RegularTile;
    use crate::domain::tiles::{Color, ColoredNumber, Number};
    use strum::IntoEnumIterator;

    fn good_run() -> (Vec<Tile>, Run) {
        let original = vec![
            RegularTile(ColoredNumber::new(Red, Six)),
            RegularTile(ColoredNumber::new(Red, Seven)),
            RegularTile(ColoredNumber::new(Red, Eight)),
        ];
        let run = Run::parse(original.clone()).unwrap();
        (original.clone(), run.clone())
    }

    #[test]
    fn decomposition_matches() {
        let (origin, run) = good_run();
        let rot = run.decompose();
        assert_eq!(origin, rot)
    }

    #[test]
    fn contains_correct() {
        let (origin, run) = good_run();
        for item in origin {
            if let RegularTile(cn) = item {
                assert!(run.contains(cn.num))
            } else {
                panic!("Test Broken!")
            }
        }
    }

    #[test]
    fn add_tile_happy_path() {
        let (origin, run) = good_run();
        let mut last_cn = ColoredNumber::get_rand();
        if let RegularTile(cn) = origin.last().unwrap() {
            if last_cn.num == Thirteen {
                // UGH randomness while testing -> try different test later
                return;
            }
            last_cn = cn.clone();
        }
        let new_tile = RegularTile(ColoredNumber::new(last_cn.color, last_cn.num.next()));
        let result = run.add_tile(&new_tile, None);
        assert!(result.is_some());
        let mut origin_plus = origin.clone();
        origin_plus.push(new_tile);
        assert_eq!(result, Run::parse(origin_plus).ok());

        let run_plus_joke = run.add_tile(&JokersWild, None);
        assert!(run_plus_joke.is_some());
        let mut origin_joke = origin.clone();
        origin_joke.push(JokersWild);
        assert_eq!(run_plus_joke, Run::parse(origin_joke).ok())
    }
}
