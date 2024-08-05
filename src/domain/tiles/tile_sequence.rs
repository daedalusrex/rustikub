use std::cmp::Ordering;
use std::collections::HashSet;

use strum::IntoEnumIterator;

use crate::domain::score_value::ScoreValue;
use crate::domain::sets::group::Group;
use crate::domain::sets::run::Run;
use crate::domain::tiles::color::Color;
use crate::domain::tiles::number::Number;
use crate::domain::tiles::Tile;
use crate::domain::tiles::Tile::{JokersWild, RegularTile};
use crate::domain::Decompose;

// https://doc.rust-lang.org/beta/reference/items/type-aliases.html
/// TYPE ALIAS: An Ordered Sequence of Tiles, such that rearranging it would change it's meaning
pub type TileSequence = Vec<Tile>;

pub fn only_regular_tiles(tiles: &TileSequence) -> TileSequence {
    return tiles
        .iter()
        .filter(|t| t.is_regular())
        .map(|t| t.clone())
        .collect();
}

pub fn unique_colors(tiles: &TileSequence) -> HashSet<Color> {
    // Wow -> https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
    return tiles.iter().filter_map(|t| t.get_color()).collect();
}

// Just for testing out type aliases

impl Decompose for TileSequence {
    fn decompose(&self) -> TileSequence {
        return self.clone();
    }
}

impl TileSequenceType {
    /// Returns the same sequence of tiles, in order, but only with tiles that match the color
    pub fn filter_color(&self, color: Color) -> TileSequence {
        self.0
            .iter()
            .filter_map(|t| {
                // This syntax is ugly, but I don't want to fight rustfmt
                // below filter then map is cleaner
                if t.is_color(&color) {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn filter_number(&self, num: Number) -> TileSequence {
        // I did it this way elsewhere. Whoops
        self.0
            .iter()
            .filter(|&t| t.is_number(&num))
            .map(|t| t.clone())
            .collect()
    }

    /// Exactly the same as the Rack get_largest_run, but New!
    /// Also public.
    /// TODO Current Implementation Ignores Jokers  -> Fix will be to increase search space by inserting them
    pub fn largest_possible_run(&self) -> Option<Run> {
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
                    .filter_map(Run::parse)
                    .collect::<Vec<Run>>(), // Not sure why needed but okay
            );
        }
        let mut valid_runs: Vec<Run> = optional_runs
            .iter()
            .filter_map(|r| Some(r.clone()))
            .collect();
        let largest_run = highest_value_collection(&mut valid_runs);
        largest_run.and_then(|run| Some(run.clone()))
    }
}

// An alternative implementation using the "New Type Idiom"
// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
/// NEWTYPE: An Ordered Sequence of Tiles, such that rearranging it would change it's meaning
pub struct TileSequenceType(pub Vec<Tile>);

impl Decompose for TileSequenceType {
    fn decompose(&self) -> TileSequence {
        return self.0.decompose();
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

fn ordering_closure<T: Decompose>(_self: &T, _other: &T) -> Ordering {
    // I don't know what && means, but ChatGPT says it works so....
    let self_score = ScoreValue::add_em_up(&_self.decompose());
    let other_score = ScoreValue::add_em_up(&_other.decompose());
    return Ord::cmp(&self_score, &other_score);
}

/// Ranks a given set of Tile Sequences (or what have you) by their Scores
/// Highest Value is first.
/// TODO probably a way to do this by implementing the partial order trait and breaking this up...
pub fn highest_value_collection<T: Decompose>(collections: &mut Vec<T>) -> Option<&T> {
    // An inner function, No need for a closure since it doesn't use anything in the outer scope
    collections.sort_by(ordering_closure);
    collections.last()

    // TODO the idiomatic way which require Ord for (ScoreValue, &T)
    // How to implement traits for "adhoc" tuples? -> Automatically present of Ord is on each item in the tuple
    // let max_value = sequences
    //     .iter()
    //     .map(|&col| (ScoreValue::add_em_up(&col.decompose()), col))
    //     .max();
    // let (_, col) = max_value?;
    // Some(col)
    // let mut max_value = ScoreValue::of(0);
    // let mut max_col: &T = collections[0];
    // for &col in collections {
    //     let col_value = ScoreValue::add_em_up(&col.decompose());
    //     if col_value > max_value {
    //         max_value = col_value;
    //         max_col = &col;
    //     }
    // }
    // return Some(max_col);
}

#[cfg(test)]
#[deny(unused_imports)]
mod sequence_tests {
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
        let mut base_vec = vec![base_case];
        let actual = highest_value_collection(&mut base_vec);
        assert_eq!(Some(&base_case), actual);
        println!("actual: {:?}", actual);
        // TODO some day, dynamic arrays of boxes would be cool
        // require Box<dyn Trait>
        // let real_group = Group::of(&Two, &vec![Red, Blue, Black]).expect("BROKEN");
        // let real_case: Vec<Box<&dyn Decompose>> = vec![Box::new(&base_case), Box::new(&real_group)];
        // let wowza = highest_value_collection(real_case);
        let twos = Group::of(&Two, &vec![Red, Blue, Black]).expect("BROKEN");
        let fours = Group::of(&Four, &vec![Red, Blue, Black]).expect("BROKEN");
        let mut group_vec = vec![twos, fours.clone()];
        let actual = highest_value_collection(&mut group_vec);
        assert_eq!(Some(&fours), actual);
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
}
