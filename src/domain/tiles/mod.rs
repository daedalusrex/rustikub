pub mod color;
pub mod number;

use crate::domain::score_value::ScoreValue;
use crate::domain::sets::run::Run;
use crate::domain::Decompose;
use color::Color;
use colored::ColoredString;
use colored::Colorize;
use number::Number;
use rand::seq::IteratorRandom;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use Tile::{JokersWild, RegularTile};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Tile {
    JokersWild,
    RegularTile(Color, Number),
}

impl Decompose for Tile {
    /// can a tile decompose into itself? -> YES lol
    fn decompose(&self) -> TileSequence {
        return vec![self.clone()];
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text: ColoredString;
        match self {
            JokersWild => text = "J".bright_green(),
            RegularTile(color, num) => {
                let num_str: String = num.as_value().to_string();
                match color {
                    // Could do this as a color implementation...
                    Color::Red => text = num_str.red(),
                    Color::Blue => text = num_str.blue(),
                    Color::Orange => text = num_str.yellow(),
                    Color::Black => text = num_str.white(),
                }
            }
        };
        write!(f, "{} ", text.bold().on_black())
    }
}

pub fn only_regular_tiles(tiles: &Vec<Tile>) -> TileSequence {
    // TODO, somehow maybe use where, to bound the return type to only show regular tiles
    return tiles
        .iter()
        .filter(|t| t.is_regular())
        .map(|t| t.clone())
        .collect();
}

pub fn unique_colors(tiles: &TileSequence) -> HashSet<Color> {
    // WOW filter_map is awesome
    // -> https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
    return tiles.iter().filter_map(|t| t.get_color()).collect();
}

// Just for testing out type aliases
// https://doc.rust-lang.org/beta/reference/items/type-aliases.html
/// TYPE ALIAS: An Ordered Sequence of Tiles, such that rearranging it would change it's meaning
pub type TileSequence = Vec<Tile>;

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

impl Tile {
    pub fn any_regular() -> Self {
        RegularTile(Color::get_rand(), Number::get_rand())
    }

    pub fn all_unique_numbered() -> Vec<Self> {
        let mut unique: Vec<Tile> = Vec::new();
        for color in Color::iter() {
            for num in Number::iter() {
                unique.push(RegularTile(color, num))
            }
        }
        return unique;
    }

    pub fn is_color(&self, color: &Color) -> bool {
        match self {
            JokersWild => false,
            RegularTile(c, _) => c == color,
        }
    }

    /// Gets the color if it's a regular tile, Jokers have no color
    pub fn get_color(&self) -> Option<Color> {
        match self {
            JokersWild => None,
            RegularTile(col, _) => Some(*col),
        }
    }

    /// Gets the color if it's a regular tile, Jokers have no number
    pub fn get_number(&self) -> Option<Number> {
        match self {
            JokersWild => None,
            RegularTile(_, num) => Some(*num),
        }
    }

    pub fn is_number(&self, num: &Number) -> bool {
        match self {
            JokersWild => false,
            RegularTile(_, n) => n == num,
        }
    }

    pub fn is_joker(&self) -> bool {
        if let JokersWild = self {
            return true;
        }
        false
    }

    pub fn is_regular(&self) -> bool {
        !self.is_joker()
    }
}

#[cfg(test)]
mod tile_tests {
    use super::{
        highest_value_collection, list_all_subsequences, unique_colors, Tile, TileSequenceType,
    };
    use crate::domain::sets::group::Group;
    use crate::domain::tiles::color::Color;
    use crate::domain::tiles::color::Color::*;
    use crate::domain::tiles::number::Number;
    use crate::domain::tiles::number::Number::*;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use colored::Colorize;
    use std::cmp::Ordering;
    use std::collections::HashSet;
    use strum::{EnumCount, IntoEnumIterator};

    #[test]
    fn correct_cardinality() {
        assert_eq!(Color::COUNT, 4);
        assert_eq!(Number::COUNT, 13)
    }

    #[test]
    fn property_confirmation() {
        let some_tile = RegularTile(Red, Twelve);
        assert!(some_tile.is_color(&Red));
        assert_eq!(some_tile.is_color(&Black), false);
        assert!(some_tile.is_number(&Twelve));
        assert_eq!(some_tile.is_number(&One), false);
        assert_eq!(some_tile.is_joker(), false);
        assert!(Tile::JokersWild.is_joker());
    }

    #[test]
    fn number_ordering() {
        assert!(One < Two);
        assert!(Two < Thirteen);
        let mut prev = One;
        for num in Number::iter() {
            if num == One {
                continue;
            } else {
                assert!(prev < num);
                prev = num;
            }
        }
    }

    #[test]
    fn color_equality() {
        assert_ne!(Red, Black);
        assert_ne!(Blue, Orange);
    }

    #[test]
    fn colored_number_comparisons() {
        let red5 = RegularTile(Red, Five);
        let blue8 = RegularTile(Blue, Eight);
        assert_ne!(red5, blue8);
        let alt_red5 = RegularTile(Red, Five);
        assert_eq!(red5, alt_red5);
    }

    #[test]
    fn many_ways_to_filter() {}

    #[test]
    fn random_tiles() {
        let rt = Tile::any_regular();
        match rt {
            Tile::JokersWild => {
                assert!(false)
            }
            RegularTile(_, _) => {
                assert!(true)
            }
        }
    }

    #[test]
    fn all_unique() {
        //Assert that this gives all unique regular tiles. i.e. 13x4 -> 52
        let foo = Tile::all_unique_numbered();
        assert_eq!(foo.len(), 52)
    }

    #[test]
    fn display_pretty_print() {
        println!("Now printing very nice display of all unique normal tiles");
        let all = Tile::all_unique_numbered();

        for t in all {
            print!("{}", t)
        }
        print!("{}", Tile::JokersWild);
        println!()
    }

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

    /// Not directly relevant for Rummikub, but works, and demonstrates how to
    /// create a sorting behavior for a vector of vectors
    /// Since it is more difficult to implement the ord trait for vectors since I don't own that crate
    /// There may be a way to do it with associated types?
    /// see: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
    #[test]
    fn sratch_sorting_vectors_against_vectors() {
        let foo = vec![1, 2, 3];
        let bar = vec![1, 2];
        let baz = vec![];

        let mut foobar = vec![foo, bar, baz];
        let my_clos = |one: &Vec<i32>, other: &Vec<i32>| -> Ordering {
            if one.len() < other.len() {
                return Ordering::Less;
            } else if one.len() == other.len() {
                return Ordering::Equal;
            }
            Ordering::Greater
        };

        foobar.sort_by(my_clos);
        println!("foobar {:?}", foobar)
    }

    #[test]
    fn test_unique_colors_happy_path() {
        let two_tiles = vec![
            RegularTile(Red, One),
            RegularTile(Blue, Twelve),
            RegularTile(Blue, One),
        ];
        let actual = unique_colors(&two_tiles);
        let mut expected: HashSet<Color> = HashSet::new();
        expected.insert(Red);
        expected.insert(Blue);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_unique_colors_jokers() {
        let two_tiles = vec![
            JokersWild,
            RegularTile(Red, One),
            JokersWild,
            RegularTile(Blue, Twelve),
        ];
        let actual = unique_colors(&two_tiles);
        let mut expected: HashSet<Color> = HashSet::new();
        expected.insert(Red);
        expected.insert(Blue);
        assert_eq!(actual, expected)
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
