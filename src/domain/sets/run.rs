use super::ParseError::*;
use crate::domain::score_value::ScoringRule::OnRack;
use crate::domain::score_value::{ScoreValue, ScoringRule};
use crate::domain::tiles::color::Color;
use crate::domain::tiles::number::Number;
use crate::domain::tiles::tile_sequence::{unique_colors, TileSequence};
use crate::domain::tiles::Tile;
use crate::domain::tiles::Tile::{JokersWild, RegularTile};
use crate::domain::{Decompose, RummikubError};
use std::collections::HashSet;
use std::vec;
use ScoringRule::OnTable;

const MAX_RUN_SIZE: usize = 13;
const MIN_RUN_SIZE: usize = 3;
const MIN_NATURAL_RUN_SPLIT_SIZE: usize = 6;
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
    pub fn of(start: Number, color: Color, len: u16) -> Option<Run> {
        if len < MIN_RUN_SIZE as u16 {
            return None;
        }
        let start_val = start.as_value();
        if start_val + ScoreValue::of_u16(len - 1) > ScoreValue::of_u16(13) {
            return None;
        }
        let mut end = start;
        for _ in 0..len - 1 {
            end = end.next()?
        }
        Some(Run {
            start,
            end,
            color,
            jokers: HashSet::new(),
        })
    }

    /// Parses a reference to an immutable vector of tiles, i.e. TileSequence, and check it is
    /// a valid run, based on the rules of Rummikub. If it is create the run, and return it
    /// otherwise returns None.
    /// This assumes the order given is the order intended, and does not try any other permutations
    /// or orderings. It also assumes the Jokers are not intended to be moved around.
    /// Updated to take slice because a vector coerces down to a slice
    /// https://doc.rust-lang.org/book/ch04-03-slices.html
    pub fn parse(candidates: &[Tile]) -> Option<Run> {
        if candidates.len() < MIN_RUN_SIZE || candidates.len() > MAX_RUN_SIZE {
            return None;
        }

        let color_set = unique_colors(candidates);
        // No Colors, or more than one distinct color
        if color_set.len() != 1 {
            return None;
        }
        let color = *color_set.iter().next()?;

        let joker_count = candidates.iter().filter(|t| t.is_joker()).count();
        if joker_count > MAX_JOKERS_IN_RUN {
            return None;
        }

        // Ignoring jokers what is the first number in the candidates
        let first: Number = candidates.iter().filter_map(|t| t.get_number()).next()?;
        // Let's assume that the first tile in the candidates is a regular tile
        let mut start: Number = first;

        // Find that regular number's location in the candidates, anything other than 0 must be jokers
        let first_position = candidates.iter().position(|t| t.is_number(first))?;
        // If we have joker(s) in front, we need to begin our hypothetical run
        // at the number that the joke(r) represents
        for i in 0..first_position {
            start = start.prev()? // If there is no previous than it's not a valid sequence
        }

        let mut expected_current: Option<Number> = Some(start);
        let mut jokers: HashSet<Number> = HashSet::new();
        let mut end: Number = start;
        for tile in candidates {
            let expected = expected_current?;
            match tile {
                JokersWild => {
                    jokers.insert(expected);
                }
                RegularTile(_, num) => {
                    if *num != expected {
                        return None;
                    }
                }
            }
            end = expected;
            expected_current = expected.next();
        }

        Some(Run {
            start,
            end,
            color,
            jokers,
        })
    }

    /// Returns the leftmost (i.e. smallest) tile that could be added to this run
    /// so if run is 2,3,4 -> 1.
    fn leftmost_open_slot(&self) -> Option<Tile> {
        let left = self.start.prev()?;
        Some(RegularTile(self.color, left))
    }

    /// Returns the rightmost (i.e largest) tile that could be added to this run
    /// so if run is 10,11,12 -> 13
    fn rightmost_open_slot(&self) -> Option<Tile> {
        let right = self.end.next()?;
        Some(RegularTile(self.color, right))
    }

    /// Returns open slots where tiles could be attached on either end
    pub fn open_slots(self) -> Option<HashSet<Tile>> {
        let optional_slots = vec![self.leftmost_open_slot(), self.rightmost_open_slot()];

        let slots: HashSet<Tile> = optional_slots
            .into_iter()
            .filter(|s| s.is_some())
            .map(|t| t.unwrap())
            .collect::<HashSet<Tile>>();
        if slots.len() == 0 {
            return None;
        }
        Some(slots)
    }

    /// Returns all possible pairs of runs that can be created by splitting a single run
    /// in two, without adding any additional tiles.
    /// Gives a vector, because that will allow strategy to consider max possible potential
    /// spots to insert a new tile
    /// Indexes/position are the only thing that matters for this kind of split
    pub fn all_possible_natural_splits(&self) -> Option<Vec<(Run, Run)>> {
        let tiles = self.decompose();

        if tiles.len() < MIN_NATURAL_RUN_SPLIT_SIZE {
            return None;
        }
        let mut run_pairs: Vec<(Run, Run)> = vec![];

        let first_split = MIN_RUN_SIZE;
        let max_split = tiles.len() + 1 - first_split; // todo verify not potential off by one error

        for mid in first_split..max_split {
            // SLICED AND DICED -> No copy, more efficient
            let (left, right) = tiles.split_at(mid);
            run_pairs.push((Run::parse(left)?, Run::parse(right)?))
        }
        if run_pairs.len() == 0 {
            return None;
        }
        Some(run_pairs)
    }

    pub fn contains(&self, n: Number) -> bool {
        self.start <= n && self.end >= n
    }

    pub fn get_run_color(&self) -> Color {
        self.color
    }

    /// takes a candidate tile. If it is possible and allowed to be added returns a NEW run
    /// with the tile attached. Requested Spot is only considered for Jokers, which could be placed
    /// on either end of the run. If none is provided the highest value location will be chosen
    pub fn add_tile(&self, tile: &Tile, requested_spot: Option<&Number>) -> Option<Self> {
        // TODO Consider breaking this up into different types of functions, simple ones first, joker later
        // TODO, honestly consider throwing away/re-writing, lots of different ideas crammed in here

        // Clojure logic for where to put Joker, only if requested spot is not provided
        let find_highest_target = || -> Option<Number> {
            return if self.end == Number::Thirteen {
                if self.start == Number::One {
                    None // Ridiculous but possible edge case
                } else {
                    Some(self.start.prev()?)
                }
            } else {
                Some(self.end.next()?)
            };
        };

        let is_new_location_valid = |num: Option<&Number>| -> bool {
            match num {
                None => false,
                Some(num) => {
                    let success;
                    let condition_one = num != &self.end && num != &self.start;
                    let mut condition_two_or1: bool = false;
                    if let Some(next_num) = &self.end.next() {
                        condition_two_or1 = num == next_num
                    }
                    let mut condition_two_or2: bool = false;
                    if let Some(prev_num) = &self.start.prev() {
                        condition_two_or2 = prev_num == num;
                    }
                    success = condition_one && (condition_two_or1 || condition_two_or2);
                    return success;
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
        let mut current = Some(self.start);
        let mut tiles: Vec<Tile> = vec![];

        while current.is_some() && current.unwrap() <= self.end {
            let num = current.unwrap();
            if self.jokers.contains(&num) {
                tiles.push(JokersWild);
            } else {
                tiles.push(RegularTile(self.color, num));
            }
            current = current.unwrap().next();
        }
        tiles
    }

    fn score(&self, rule: ScoringRule) -> Result<ScoreValue, RummikubError> {
        match rule {
            OnRack => self.decompose().score(OnRack),
            OnTable => {
                let mut current = Some(self.start);
                let mut sum: ScoreValue = ScoreValue::of_u16(0u16);
                while current.is_some() && current.unwrap() <= self.end {
                    let number = current.unwrap();
                    sum += number.as_value();
                    current = number.next();
                }
                Ok(sum)
            }
        }
    }
}

#[cfg(test)]
mod run_parsing {
    use super::*;
    use crate::domain::score_value::JOKER_RACK_SCORE;
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
            RegularTile(color, first.next().expect("BOOM")),
            RegularTile(color, first.next().expect("BOOM").next().expect("BOOM")),
        ]
    }

    #[test]
    fn parse_happy_path() {
        let happy = object_mother_good_run_of_three();
        let first_tile = happy.first().unwrap().clone();
        if let RegularTile(color, num) = first_tile {
            let result = Run::parse(&happy);
            assert!(result.is_some());
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
        assert!(Run::parse(&okay1.clone()).is_some());
        let okay2 = vec![JokersWild, first, second, third];
        assert!(Run::parse(&okay2.clone()).is_some());
        assert_eq!(okay2, Run::parse(&okay2.clone()).unwrap().decompose());
        let okay3 = vec![JokersWild, JokersWild, first, second, third];
        assert!(Run::parse(&okay3.clone()).is_some());
        assert_eq!(okay3, Run::parse(&okay3.clone()).unwrap().decompose());
        let okay4 = vec![first, second, third, JokersWild, JokersWild];
        assert!(Run::parse(&okay4.clone()).is_some());
        assert_eq!(okay4, Run::parse(&okay4.clone()).unwrap().decompose());
    }

    #[test]
    fn too_many_jokers_at_the_end() {
        let color = Color::get_rand();
        let first = RegularTile(color, Three);
        let second = RegularTile(color, Four);
        let third = RegularTile(color, Five);
        let too_many = vec![first, second, third, JokersWild, JokersWild, JokersWild];
        assert!(Run::parse(&too_many.clone()).is_none());
    }

    #[test]
    fn reject_bad_joker_edges_of_run() {
        let color = Color::get_rand();
        let first = RegularTile(color, Three);
        let second = RegularTile(color, Four);
        let third = RegularTile(color, Five);
        let success = vec![first, second, third, JokersWild];
        assert!(Run::parse(&success.clone()).is_some());

        let color = Color::get_rand();
        let first = RegularTile(color, Three);
        let second = RegularTile(color, Four);
        let third = RegularTile(color, Five);
        let success = vec![JokersWild, first, second, third];
        assert!(Run::parse(&success.clone()).is_some());
    }

    #[test]
    fn parse_failure_cases_quantity() {
        let happy = object_mother_good_run_of_three();
        let first_tile = happy.first().unwrap().clone();
        if let RegularTile(color, num) = first_tile {
            //too few
            let mut too_few = happy.clone();
            too_few.pop();
            assert!(Run::parse(&too_few.clone()).is_none());
            // Can also Specify Error Type
            assert_eq!(None, Run::parse(&too_few));

            //too many
            let mut too_many = happy.clone();
            for num in Number::iter() {
                too_many.push(RegularTile(color, num));
            }
            assert!(Run::parse(&too_many).is_none());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn parse_one_distinct_color() {
        let mut distinct_colors = object_mother_good_run_of_three();
        let first_tile = distinct_colors.first().unwrap().clone();
        if let RegularTile(color, num) = first_tile {
            // This failed once, at the expectation, but probably just the test code...
            distinct_colors.push(RegularTile(color.next(), num.prev().unwrap_or(One)));
            assert!(Run::parse(&distinct_colors).is_none());
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
            assert!(Run::parse(&dupped).is_none());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn rejects_reversed_ordering() {
        let mut reversed = object_mother_good_run_of_three();
        reversed.reverse();
        assert!(Run::parse(&reversed).is_none());
    }

    #[test]
    fn rejects_1_after_13() {
        let color = Color::get_rand();
        let first = RegularTile(color, Twelve);
        let second = RegularTile(color, Thirteen);
        let third = RegularTile(color, One);
        let end_at_13: Vec<Tile> = vec![first, second, third];
        assert!(Run::parse(&end_at_13).is_none());
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
        assert!(Run::parse(&random_order).is_none());
    }

    #[test]
    fn score_value_is_correct() {
        let color = Color::get_rand();
        let first = RegularTile(color, Five);
        let second = RegularTile(color, Six);
        let third = RegularTile(color, Seven);

        let expected_sum = ScoreValue::of(5 + 6 + 7);
        let known_run = Run::parse(&vec![first, second, third]).expect("BROKEN");
        assert_eq!(expected_sum, known_run.score(OnTable));
        assert_eq!(expected_sum, known_run.score(OnRack));

        let actual_sum_with_joker = ScoreValue::of_u16(5 + 6 + 7 + 8);
        let with_joker = Run::parse(&vec![first, second, third, JokersWild]).expect("BROKEN");
        assert_eq!(actual_sum_with_joker, with_joker.score(OnTable).unwrap());
        assert_eq!(
            expected_sum.unwrap() + JOKER_RACK_SCORE,
            with_joker.score(OnRack).unwrap()
        );
    }
}

#[cfg(test)]
mod other_tests_of_runs {
    use super::*;
    use crate::domain::sets::ParseError::*;
    use crate::domain::tiles::color::Color::*;
    use crate::domain::tiles::number::Number::*;
    use crate::domain::tiles::Tile::RegularTile;
    use std::hash::Hash;

    fn good_run_red_678() -> (Vec<Tile>, Run) {
        let original = vec![
            RegularTile(Red, Six),
            RegularTile(Red, Seven),
            RegularTile(Red, Eight),
        ];
        let run = Run::parse(&original.clone()).unwrap();
        (original.clone(), run.clone())
    }

    #[test]
    fn decomposition_matches() {
        let (origin, run) = good_run_red_678();
        let rot = run.decompose();
        assert_eq!(origin, rot)
    }

    #[test]
    fn contains_correct() {
        let (origin, run) = good_run_red_678();
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
        let (origin, run) = good_run_red_678();
        let origin_tile = origin.last().expect("BROKEN");
        if let RegularTile(color, num) = origin_tile {
            if num == &Thirteen {
                // TODO UGH randomness while testing -> try different test later
                return;
            }

            let new_tile = RegularTile(*color, num.next().expect("BROKEN TEST"));
            let result = run.add_tile(&new_tile, None);
            assert!(result.is_some());
            let mut origin_plus = origin.clone();
            origin_plus.push(new_tile);
            assert_eq!(result, Run::parse(&origin_plus));

            let run_plus_joke = run.add_tile(&JokersWild, None);
            assert!(run_plus_joke.is_some());
            let mut origin_joke = origin.clone();
            origin_joke.push(JokersWild);
            assert_eq!(run_plus_joke, Run::parse(&origin_joke))
        } else {
            assert!(false)
        }
    }

    #[test]
    fn known_infinite_loop_edge_case() {
        use std::thread;
        use std::time::Duration;
        use Color::*;
        let special_case: Vec<Tile> = vec![
            RegularTile(Black, Eleven),
            RegularTile(Black, Twelve),
            RegularTile(Black, Thirteen),
        ];

        let thread_handle = thread::spawn(move || {
            let result = Run::parse(&special_case);
            assert!(result.is_some());
            let rotten = result.unwrap().decompose(); // The infinite loop was in decompose
            assert_eq!(rotten.len(), 3);
        });

        thread::sleep(Duration::from_millis(10));
        if !thread_handle.is_finished() {
            // The test should finish nearly instantly in sub-milliseconds so waiting
            // this long indicates it's stuck in an infinite loop
            panic!("Parse test took way too long! There must be an infinite loop!")
        }
    }

    #[test]
    pub fn of_cardinality() {
        let run = Run::of(One, Blue, 3).expect("BROKEN");
        let mut expected: TileSequence = vec![
            RegularTile(Blue, One),
            RegularTile(Blue, Two),
            RegularTile(Blue, Three),
        ];
        expected.sort();
        let mut actual = run.decompose();
        actual.sort();

        assert_eq!(actual.len(), 3);
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn run_scoring() {
        let run_start = Run::of(One, Blue, 3).unwrap();
        assert_eq!(run_start.score(OnRack).unwrap(), ScoreValue::of_u16(6));
        let run_end = Run::of(Eleven, Blue, 3).unwrap();
        assert_eq!(run_end.score(OnTable).unwrap(), ScoreValue::of_u16(36));

        let run_joker: Run = Run::parse(&vec![
            JokersWild,
            RegularTile(Blue, Two),
            RegularTile(Blue, Three),
            JokersWild,
        ])
            .unwrap();
        assert_eq!(run_joker.score(OnRack).unwrap(), ScoreValue::of_u16(65));
        assert_eq!(run_joker.score(OnTable).unwrap(), ScoreValue::of_u16(10));
    }

    #[test]
    pub fn test_left_right_open_slots() {
        let one_two_three: Run = Run::of(One, Blue, 3).unwrap();
        let eleven_twelve_thirteen: Run = Run::of(Eleven, Black, 3).unwrap();

        assert_eq!(one_two_three.leftmost_open_slot(), None);
        assert_eq!(
            one_two_three.rightmost_open_slot(),
            Some(RegularTile(Blue, Four))
        );

        assert_eq!(
            eleven_twelve_thirteen.leftmost_open_slot(),
            Some(RegularTile(Black, Ten))
        );
        assert_eq!(eleven_twelve_thirteen.rightmost_open_slot(), None);
    }

    #[test]
    pub fn test_open_slots() {
        let one_thru_thirteen: Run = Run::of(One, Orange, 13).unwrap();
        let one_two_three: Run = Run::of(One, Blue, 3).unwrap();
        let two_three_four: Run = Run::of(Two, Blue, 3).unwrap();
        let eleven_twelve_thirteen: Run = Run::of(Eleven, Black, 3).unwrap();

        assert_eq!(one_thru_thirteen.open_slots(), None);

        let expected = Some(HashSet::from([RegularTile(Blue, Four)]));
        assert_eq!(one_two_three.open_slots(), expected);

        let expected = Some(HashSet::from([
            RegularTile(Blue, One),
            RegularTile(Blue, Five),
        ]));
        assert_eq!(two_three_four.open_slots(), expected);

        let expected = Some(HashSet::from([RegularTile(Black, Ten)]));
        assert_eq!(eleven_twelve_thirteen.open_slots(), expected);
    }

    #[test]
    pub fn test_all_possible_natural_splits() {
        let one_two_three: Run = Run::of(One, Blue, 3).unwrap();
        assert_eq!(one_two_three.all_possible_natural_splits(), None);

        let two_thru_seven: Run = Run::of(Two, Blue, 6).unwrap();
        let expected: Vec<(Run, Run)> = vec![(
            Run::of(Two, Blue, 3).unwrap(),
            Run::of(Five, Blue, 3).unwrap(),
        )];
        assert_eq!(
            two_thru_seven.all_possible_natural_splits().unwrap(),
            expected
        );

        let three_thru_ten: Run = Run::of(Three, Blue, 8).unwrap();
        let expected: Vec<(Run, Run)> = vec![
            (
                Run::of(Three, Blue, 3).unwrap(),
                Run::of(Six, Blue, 5).unwrap(),
            ),
            (
                Run::of(Three, Blue, 4).unwrap(),
                Run::of(Seven, Blue, 4).unwrap(),
            ),
            (
                Run::of(Three, Blue, 5).unwrap(),
                Run::of(Eight, Blue, 3).unwrap(),
            ),
        ];
        assert_eq!(
            three_thru_ten.all_possible_natural_splits().unwrap(),
            expected
        )
    }
}
