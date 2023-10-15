use super::ParseError;
use super::ParseError::*;
use crate::domain::score_value::ScoreValue;
use crate::domain::tiles::color::Color;
use crate::domain::tiles::number::Number;
use crate::domain::tiles::Tile;
use crate::domain::tiles::Tile::{JokersWild, RegularTile};
use crate::domain::Decompose;
use std::collections::HashSet;
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
    pub fn of(start: Number, color: Color, len: u8) -> Option<Run> {
        if len < MIN_RUN_SIZE as u8 {
            return None;
        }
        let start_val = start.as_value();
        if start_val + ScoreValue::of(len) > ScoreValue::of(13) {
            return None;
        }
        let mut end = start;
        for _ in 0..len {
            end = end.next();
        }
        Some(Run {
            start,
            end,
            color,
            jokers: HashSet::new(),
        })
    }

    // Using the Result<T, E> type instead of Option here. It's better suited for this? than Option
    //  https://doc.rust-lang.org/rust-by-example/error/result.html
    // TODO this should be &Vec<Tile>, a reference instead of a borrow
    // TODO This should totally be an option? Many tiles are not Runs? -> The error is nice to have I guess ???
    // TODO Honestly, just rewrite this from scratch. I Suppose I don't shuffle them right? I expect it to be ordered already?? Could sort pretty easily...
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
        let mut first_definitive_cn: Option<Number> = None;
        let mut previous = Number::Thirteen;

        // TODO Alternate approach, acquire the index, and do prepend based on that -> hmm
        // candidates.iter().enumerate().filter()
        // let mut iter_of_only_regular = candidates.iter().filter(|tile| !tile.is_joker());
        // let first_no_joke = iter_of_only_regular.next().ok_or(IllegalJokers);

        // closure logic checks
        // TODO man I have no idea what you were doing here but it's not correct, why not just compare tiles?
        // TODO, I think this closure should be -> tiles can be adjaacent?
        let validate_sequence_with_next =
            |next: &Tile, prev: Number, possible_color: Color| -> Result<Number, ParseError> {
                if prev == Number::Thirteen {
                    return Err(OutOfBounds);
                }
                if next.is_joker() && prev < Number::Thirteen {
                    return Ok(prev.next());
                }
                if next.is_number(&prev) {
                    return Err(DuplicateNumbers);
                }
                if !next.is_number(&prev.next()) {
                    return Err(OutOfOrder);
                }
                if !next.is_color(&possible_color) {
                    return Err(DistinctColors);
                }
                Ok(prev.next())
            };

        // Go through the vector and check the constraints
        for (idx, tile) in candidates.iter().enumerate() {
            match tile {
                JokersWild => match first_definitive_cn {
                    // TODO this is definitely wrong if you start with a Joker, which is totally valid
                    Some(first_cn) => {
                        let next = validate_sequence_with_next(tile, previous, color)?;
                        jokers.insert(next);
                        end = next;
                        previous = next;
                    }
                    None => prepend_joker_count += 1,
                },
                RegularTile(regular_color, regular_number) => {
                    match first_definitive_cn {
                        Some(first_cn) => {
                            let next = validate_sequence_with_next(tile, previous, color)?;
                            end = next; // Keep incrementing the newly found end representation
                            previous = *regular_number; // TODO what does derefrence do for copy?
                        }
                        None => {
                            // Only happens once, when we first encounter a regular tile
                            first_definitive_cn = Some(regular_number.clone());
                            start = regular_number.clone();
                            color = regular_color.clone();
                            previous = regular_number.clone(); // Is this a move or a copy?
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
    pub fn add_tile(&self, tile: &Tile, requested_spot: Option<&Number>) -> Option<Self> {
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

        let is_new_location_valid = |num: Option<&Number>| -> bool {
            match num {
                None => false,
                Some(num) => {
                    return (num != &self.end && num != &self.start)
                        && (num == &self.end.next() || num == &self.start.prev())
                }
            }
        };

        let new_delimiters = |cand: &Number| -> (Number, Number) {
            let mut new_start = self.start;
            let mut new_end = self.end;
            if cand > &self.end {
                new_end = cand.clone();
            } else if cand < &self.start {
                new_start = cand.clone();
            }
            return (new_start, new_end);
        };

        let update_jokers = |num: &Number| {
            let mut new_jokers = self.jokers.clone();
            new_jokers.insert(num.clone());
            new_jokers
        };

        match tile {
            JokersWild => {
                if self.jokers.len() >= MAX_JOKERS_IN_RUN {
                    return None;
                } else if is_new_location_valid(requested_spot) {
                    let req = requested_spot?;
                    let (new_start, new_end) = new_delimiters(req);
                    let new_jokers = update_jokers(req);
                    return Some(Run {
                        start: new_start,
                        end: new_end,
                        jokers: new_jokers,
                        color: self.color,
                    });
                } else if requested_spot.is_none() {
                    let new_spot = find_highest_target()?;
                    let (new_start, new_end) = new_delimiters(&new_spot);
                    let new_jokers = update_jokers(&new_spot);
                    return Some(Run {
                        start: new_start,
                        end: new_end,
                        jokers: new_jokers,
                        color: self.color,
                    });
                }
                return None;
            }
            RegularTile(color, num) => {
                if color != &self.color || requested_spot.is_some() {
                    return None;
                } else if is_new_location_valid(Some(num)) {
                    let (new_start, new_end) = new_delimiters(num);
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
                tiles.push(RegularTile(self.color, current));
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
    use crate::domain::tiles::color::Color;
    use crate::domain::tiles::number::Number;
    use crate::domain::tiles::number::Number::*;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use strum::IntoEnumIterator;

    pub fn object_mother_good_run_of_three() -> Vec<Tile> {
        let mut first = Number::get_rand();
        let color = Color::get_rand();
        // Questionable
        if first > Eleven {
            first = Eleven
        }
        vec![
            RegularTile(color, first),
            RegularTile(color, first.next()),
            RegularTile(color, first.next().next()),
        ]
    }

    #[test]
    fn parse_happy_path() {
        let happy = object_mother_good_run_of_three();
        let first_tile = happy.first().unwrap().clone();
        if let RegularTile(color, num) = first_tile {
            let result = Run::parse(happy);
            assert!(result.is_ok());
            let success = result.expect("BROKEN");
            assert_eq!(success.start, num);
            assert_eq!(success.color, color);
            // TODO more precise success metrics
        } else {
            assert!(false)
        }
    }

    #[test]
    fn proper_joker_handling() {
        let color = Color::get_rand();
        let first = RegularTile(color, Three);
        let second = RegularTile(color, Four);
        let third = RegularTile(color, Five);
        let okay1 = vec![first, second, third, JokersWild];
        assert!(Run::parse(okay1.clone()).is_ok());
        let okay2 = vec![JokersWild, first, second, third];
        assert!(Run::parse(okay2.clone()).is_ok());
        assert_eq!(okay2, Run::parse(okay2.clone()).unwrap().decompose());
        let okay3 = vec![JokersWild, JokersWild, first, second, third];
        assert!(Run::parse(okay3.clone()).is_ok());
        assert_eq!(okay3, Run::parse(okay3.clone()).unwrap().decompose());
        let okay4 = vec![first, second, third, JokersWild, JokersWild];
        assert!(Run::parse(okay4.clone()).is_ok());
        assert_eq!(okay4, Run::parse(okay4.clone()).unwrap().decompose());
    }

    #[test]
    fn too_many_jokers_at_the_end() {
        let color = Color::get_rand();
        let first = RegularTile(color, Three);
        let second = RegularTile(color, Four);
        let third = RegularTile(color, Five);
        let too_many = vec![first, second, third, JokersWild, JokersWild, JokersWild];
        assert!(Run::parse(too_many.clone()).is_err());
    }

    #[test]
    fn reject_bad_joker_edges_of_run() {
        let color = Color::get_rand();
        let first = RegularTile(color, Three);
        let second = RegularTile(color, Four);
        let third = RegularTile(color, Five);
        let success = vec![first, second, third, JokersWild];
        assert!(Run::parse(success.clone()).is_err());

        let color = Color::get_rand();
        let first = RegularTile(color, Three);
        let second = RegularTile(color, Four);
        let third = RegularTile(color, Five);
        let success = vec![JokersWild, first, second, third];
        assert!(Run::parse(success.clone()).is_err());
    }

    #[test]
    fn parse_failure_cases_quantity() {
        let happy = object_mother_good_run_of_three();
        let first_tile = happy.first().unwrap().clone();
        if let RegularTile(color, num) = first_tile {
            //too few
            let mut too_few = happy.clone();
            too_few.pop();
            assert!(Run::parse(too_few.clone()).is_err());
            // Can also Specify Error Type
            assert_eq!(Err(TooFewTiles), Run::parse(too_few));

            //too many
            let mut too_many = happy.clone();
            for num in Number::iter() {
                too_many.push(RegularTile(color, num));
            }
            assert!(Run::parse(too_many).is_err());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn parse_one_distinct_color() {
        let mut distinct_colors = object_mother_good_run_of_three();
        let first_tile = distinct_colors.first().unwrap().clone();
        if let RegularTile(color, num) = first_tile {
            distinct_colors.push(RegularTile(color.next(), num.prev()));
            assert!(Run::parse(distinct_colors).is_err());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn rejects_duplicate_number() {
        let mut dupped = object_mother_good_run_of_three();
        let first_tile = dupped.first().unwrap().clone();
        if let RegularTile(color, num) = first_tile {
            dupped.push(first_tile.clone());
            assert!(Run::parse(dupped).is_err());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn rejects_reversed_ordering() {
        let mut reversed = object_mother_good_run_of_three();
        reversed.reverse();
        assert!(Run::parse(reversed).is_err());
    }

    #[test]
    fn rejects_1_after_13() {
        let color = Color::get_rand();
        let first = RegularTile(color, Twelve);
        let second = RegularTile(color, Thirteen);
        let third = RegularTile(color, One);
        let end_at_13: Vec<Tile> = vec![first, second, third];
        assert!(Run::parse(end_at_13).is_err());
    }

    /// Could accidentally pass sometimes if random ordering is actually correct
    /// shrugs whatever good enough
    #[test]
    fn rejects_out_of_order_random_numbers() {
        let color = Color::get_rand();
        let first = RegularTile(color, Number::get_rand());
        let second = RegularTile(color, Number::get_rand());
        let third = RegularTile(color, Number::get_rand());
        let random_order: Vec<Tile> = vec![first, second, third];
        assert!(Run::parse(random_order).is_err());
    }

    #[test]
    fn score_value_is_correct() {
        let color = Color::get_rand();
        let first = RegularTile(color, Five);
        let second = RegularTile(color, Six);
        let third = RegularTile(color, Seven);

        let actual_sum = ScoreValue::of(5 + 6 + 7);
        let known_run = Run::parse(vec![first, second, third]).expect("BROKEN");
        assert_eq!(known_run.total_value(), known_run.total_value());

        let actual_sum_with_joker = ScoreValue::of(5 + 6 + 7 + 8);
        let with_joker = Run::parse(vec![first, second, third, JokersWild]).expect("BROKEN");
        assert_eq!(actual_sum_with_joker, with_joker.total_value());
    }
}

#[cfg(test)]
mod other_tests_of_runs {
    use super::*;
    use crate::domain::sets::ParseError::*;
    use crate::domain::tiles::color::Color::Red;
    use crate::domain::tiles::number::Number::*;
    use crate::domain::tiles::Tile::RegularTile;

    fn good_run() -> (Vec<Tile>, Run) {
        let original = vec![
            RegularTile(Red, Six),
            RegularTile(Red, Seven),
            RegularTile(Red, Eight),
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
            if let RegularTile(c, n) = item {
                assert!(run.contains(n))
            } else {
                panic!("Test Broken!")
            }
        }
    }

    #[test]
    fn add_tile_happy_path() {
        let (origin, run) = good_run();
        let mut last_number = Number::get_rand();
        let origin_tile = origin.last().expect("BROKEN");
        if let RegularTile(color, num) = origin_tile {
            if num == &Thirteen {
                // TODO UGH randomness while testing -> try different test later
                return;
            }
            last_number = num.clone();

            let new_tile = RegularTile(color.clone(), num.clone());
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
        } else {
            assert!(false)
        }
    }
}
