use std::cmp::Ordering;
use std::collections::HashSet;

use strum::IntoEnumIterator;

use crate::domain::score_value::ScoringRule::{OnRack, OnTable};
use crate::domain::score_value::{ScoreValue, ScoringRule};
use crate::domain::sets::group::Group;
use crate::domain::sets::run::Run;
use crate::domain::tiles::color::Color;
use crate::domain::tiles::number::Number;
use crate::domain::tiles::Tile;
use crate::domain::tiles::Tile::{JokersWild, RegularTile};
use crate::domain::{Decompose, RummikubError};

// https://doc.rust-lang.org/beta/reference/items/type-aliases.html
/// TYPE ALIAS: An Ordered Sequence of Tiles, such that rearranging it would change it's meaning
pub type TileSequence = Vec<Tile>;

pub fn only_regular_tiles(tiles: &TileSequence) -> TileSequence {
    tiles
        .iter()
        .filter(|t| t.is_regular())
        .map(|t| t.clone())
        .collect()
}

pub fn unique_colors(tiles: &[Tile]) -> HashSet<Color> {
    // Wow -> https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
    tiles.iter().filter_map(|t| t.get_color()).collect()
}

// Just for testing out type aliases

impl Decompose for TileSequence {
    fn decompose(&self) -> TileSequence {
        self.clone()
    }

    fn score(&self, rule: ScoringRule) -> Result<ScoreValue, RummikubError> {
        match rule {
            OnTable => Err(RummikubError),
            OnRack => Ok(ScoreValue::of(
                self.iter()
                    .map(|t| t.score(OnRack).unwrap().as_u16())
                    .sum::<u16>(),
            ))?,
        }
    }
}

// An alternative implementation using the "New Type Idiom"
// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
/// NEWTYPE: An Ordered Sequence of Tiles, such that rearranging it would change it's meaning
pub struct TileSequenceType(pub Vec<Tile>);

impl Decompose for TileSequenceType {
    fn decompose(&self) -> TileSequence {
        self.0.decompose()
    }

    fn score(&self, rule: ScoringRule) -> Result<ScoreValue, RummikubError> {
        self.0.score(rule)
    }
}
impl TileSequenceType {
    /// Simplified handy constructor for TileSequenceType
    pub fn of(tiles: &impl Decompose) -> TileSequenceType {
        TileSequenceType(tiles.decompose())
    }

    /// Returns the same sequence of tiles, in order, but only with tiles that match the color
    pub fn filter_color(&self, color: Color) -> TileSequence {
        self.0
            .iter()
            .filter(|t| t.is_color(color))
            .map(|t| t.clone())
            .collect()
    }

    /// Returns the same sequence of tiles, in order, but only with tiles matching the number
    pub fn filter_number(&self, num: Number) -> TileSequence {
        // TODO replace other instances of this by calling this one
        self.0
            .iter()
            .filter(|&t| t.is_number(num))
            .map(|t| t.clone())
            .collect()
    }

    /// Exactly the same as the Rack get_largest_run, but New!
    /// Also public.
    /// TODO Current Implementation Ignores Jokers  -> Fix will be to increase search space by inserting them
    pub fn largest_run(&self) -> Option<Run> {
        let mut optional_runs: Vec<Run> = vec![];
        // don't need to remove regular tiles cuz of cool iters above
        for color in Color::iter() {
            let mut with_color = self.filter_color(color);
            with_color.sort();
            with_color.dedup();
            let all_subsequences = list_all_subsequences(&with_color);
            // TODO insert however many jokers into each position for each sequence here
            optional_runs.extend(
                all_subsequences
                    .iter()
                    .filter_map(|arg0: &Vec<Tile>| Run::parse(arg0)) // closure required for Slice
                    .collect::<Vec<Run>>(), // Not sure why needed but okay
            );
        }
        let valid_runs = optional_runs.into_iter().filter_map(|r| Some(r)).collect();
        let largest_run = highest_value_collection(&valid_runs, ScoringRule::default());
        largest_run
    }

    pub fn largest_group(&self) -> Option<Group> {
        let mut optional_groups: Vec<Option<Group>> = vec![];
        for num in Number::iter() {
            let mut all_match_num = self.filter_number(num);
            all_match_num.dedup();
            optional_groups.push(Group::parse(all_match_num));
        }
        let largest_group = highest_value_collection(
            &optional_groups
                .into_iter()
                .flatten() // flatten is more concise .filter_map(|g| g)
                .collect::<Vec<Group>>(),
            ScoringRule::default(),
        );
        largest_group
    }

    /// Attempts to remove the given items from the tile sequence.
    /// Returns the new sequence if successful and None if not possible to remove
    /// Some(empty) is a valid response indicating all were removed
    pub fn remove(&self, items: &impl Decompose) -> Option<TileSequenceType> {
        let to_remove = items.decompose();
        let mut trimmed = self.0.clone();

        for tile in to_remove {
            if !trimmed.contains(&tile) {
                return None;
            } else {
                let pos = trimmed.iter().position(|t| t == &tile)?;
                trimmed.remove(pos);
            }
        }
        Some(TileSequenceType::of(&trimmed))
    }
}

/// Provides a sequence of all possible ordered sub-sequences of an array
/// Given 1,2 -> [(1), (1,2), (2)]
/// implemented using a sliding window, does not filter
pub fn list_all_subsequences<T>(seq: &Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut subsequences: Vec<Vec<T>> = vec![];
    for i in 0..seq.len() {
        for j in i..seq.len() {
            let sub = seq[i..=j].to_vec();
            subsequences.push(sub);
        }
    }
    return subsequences;
}

/// Ranks a given set of Tile Sequences (or what have you) by their Scores
/// Highest Value is first.
pub fn highest_value_collection<T: Decompose>(collections: &Vec<T>, rule: ScoringRule) -> Option<T>
where
    T: Clone,
{
    // Must use an actual closure since relies on the rule from the outer scope
    let ordering_closure = |left: &T, right: &T| -> Ordering {
        Ord::cmp(&left.score(rule).unwrap(), &right.score(rule).unwrap())
    };
    let mut sortable = collections.clone();
    sortable.sort_by(ordering_closure); // Magic
    sortable.last().cloned()
}

#[cfg(test)]
mod sequence_tests {
    use crate::domain::score_value::JOKER_RACK_SCORE;
    use crate::domain::tiles::color::Color::*;
    use crate::domain::tiles::number::Number::*;

    use crate::domain::tiles::Tile::{JokersWild, RegularTile};

    use super::*;

    #[test]
    fn all_contiguous_subsequences_from_generic_vec_simple() {
        let simple_case = vec![1, 2, 3];

        let mut expected = vec![
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![2, 3],
            vec![2],
            vec![3],
        ];

        expected.sort();
        let mut actual = list_all_subsequences(&simple_case);
        actual.sort();
        assert_eq!(actual, expected)
    }

    #[test]
    fn all_contiguous_subsequences_from_generic_vec_hard() {
        let simple_case = vec![1, 2, 3, 4];

        let mut expected = vec![
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 2, 3, 4],
            vec![2],
            vec![2, 3],
            vec![2, 3, 4],
            vec![3],
            vec![3, 4],
            vec![4],
        ];
        expected.sort();
        let mut actual = list_all_subsequences(&simple_case);
        actual.sort();
        assert_eq!(actual, expected)
    }

    #[test]
    fn rank_collections_correctly() {
        let base_case = RegularTile(Blue, One);
        let base_vec = vec![base_case];
        let actual = highest_value_collection(&base_vec, ScoringRule::default());
        assert_eq!(Some(base_case), actual);
        println!("actual: {:?}", actual);
        // TODO some day, dynamic arrays of boxes would be cool
        // require Box<dyn Trait>
        // let real_group = Group::of(&Two, &vec![Red, Blue, Black]).expect("BROKEN");
        // let real_case: Vec<Box<&dyn Decompose>> = vec![Box::new(&base_case), Box::new(&real_group)];
        // let wowza = highest_value_collection(real_case);
        let twos = Group::of(Two, &vec![Red, Blue, Black]).expect("BROKEN");
        let fours = Group::of(Four, &vec![Red, Blue, Black]).expect("BROKEN");
        let group_vec = vec![twos, fours.clone()];
        let actual = highest_value_collection(&group_vec, ScoringRule::default());
        assert_eq!(Some(fours), actual);
        println!("actual: {:?}", actual);
    }

    #[test]
    fn test_filters_of_tile_sequence() {
        let tiles = TileSequenceType(vec![
            RegularTile(Red, One),
            RegularTile(Blue, One),
            JokersWild,
            RegularTile(Black, One),
            RegularTile(Red, Two),
        ]);

        let color_expectation = vec![RegularTile(Red, One), RegularTile(Red, Two)];
        assert_eq!(color_expectation, tiles.filter_color(Red));

        let num_expectation = vec![
            RegularTile(Red, One),
            RegularTile(Blue, One),
            RegularTile(Black, One),
        ];
        assert_eq!(num_expectation, tiles.filter_number(One));
    }

    #[test]
    fn test_remove_from_tile_sequence() {
        let tiles = TileSequenceType(vec![
            RegularTile(Red, Twelve),
            RegularTile(Blue, One),
            RegularTile(Blue, One),
            JokersWild,
            RegularTile(Black, Ten),
            RegularTile(Black, One),
        ]);

        let expected = vec![
            RegularTile(Red, Twelve),
            RegularTile(Blue, One),
            RegularTile(Blue, One),
            RegularTile(Black, Ten),
            RegularTile(Black, One),
        ];
        assert_eq!(
            expected,
            tiles.remove(&vec![JokersWild]).expect("BROKEN").decompose()
        );

        let expected = vec![
            RegularTile(Red, Twelve),
            RegularTile(Blue, One),
            RegularTile(Blue, One),
            JokersWild,
            RegularTile(Black, Ten),
            RegularTile(Black, One),
        ];
        assert_eq!(expected, tiles.remove(&vec![]).expect("BROKEN").decompose());

        let expected = vec![
            RegularTile(Red, Twelve),
            RegularTile(Blue, One),
            JokersWild,
            RegularTile(Black, Ten),
            RegularTile(Black, One),
        ];
        assert_eq!(
            expected,
            tiles
                .remove(&vec![RegularTile(Blue, One)])
                .expect("BROKEN")
                .decompose()
        );

        let expected = vec![
            RegularTile(Red, Twelve),
            JokersWild,
            RegularTile(Black, Ten),
            RegularTile(Black, One),
        ];
        assert_eq!(
            expected,
            tiles
                .remove(&vec![RegularTile(Blue, One), RegularTile(Blue, One)])
                .expect("BROKEN")
                .decompose()
        );

        let expected = vec![
            RegularTile(Red, Twelve),
            RegularTile(Blue, One),
            RegularTile(Blue, One),
            JokersWild,
            RegularTile(Black, Ten),
            RegularTile(Black, One),
        ];
        assert!(tiles.remove(&vec![RegularTile(Orange, Five)]).is_none());

        let expected: TileSequence = vec![];
        assert_eq!(expected, tiles.remove(&tiles).expect("BROKEN").decompose());
    }

    #[test]
    fn test_largest_group() {
        let tiles = TileSequenceType(vec![
            RegularTile(Red, One),
            RegularTile(Blue, One),
            RegularTile(Black, One),
            RegularTile(Orange, One),
            RegularTile(Red, Ten),
            RegularTile(Orange, Ten),
            RegularTile(Black, Ten),
        ]);

        let actual = tiles.largest_group();

        let expectation = Group::of(Ten, &vec![Orange, Red, Black]).expect("BROKEN");
        assert!(actual.is_some());
        assert_eq!(expectation, actual.expect("BROKEN"));
    }

    #[test]
    fn test_scoring_tile_sequence() {
        let tiles: TileSequence = vec![
            RegularTile(Red, One),
            RegularTile(Blue, Three),
            RegularTile(Black, Five),
            RegularTile(Orange, One),
            RegularTile(Red, Seven),
            RegularTile(Orange, Ten),
            RegularTile(Black, Thirteen),
            JokersWild,
        ];
        let actual: ScoreValue = tiles.score(OnRack).unwrap();

        let expected: u16 = 1 + 3 + 5 + 1 + 7 + 10 + 13;
        assert_eq!(ScoreValue::of_u16(expected) + JOKER_RACK_SCORE, actual);
    }

    #[test]
    #[should_panic]
    fn test_scoring_tile_sequence_fail_ontable() {
        let tiles: TileSequence = vec![RegularTile(Red, One), JokersWild];
        let actual: ScoreValue = tiles.score(OnTable).unwrap();
    }
}
