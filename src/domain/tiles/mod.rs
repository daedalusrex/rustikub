#[deny(dead_code, unused_imports, unused_variables)]
pub mod color;
#[deny(dead_code, unused_imports, unused_variables)]
pub mod number;
pub mod tile_sequence;

use crate::domain::Decompose;
use color::Color;
use colored::ColoredString;
use colored::Colorize;
use number::Number;
use rand::seq::IteratorRandom;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use tile_sequence::TileSequence;
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
#[deny(dead_code, unused_imports, unused_variables)]
mod tile_tests {
    use super::Tile;
    use crate::domain::tiles::color::Color;
    use crate::domain::tiles::color::Color::*;
    use crate::domain::tiles::number::Number::*;
    use crate::domain::tiles::tile_sequence::unique_colors;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use std::cmp::Ordering;
    use std::collections::HashSet;
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
    fn random_tiles() {
        let rt = Tile::any_regular();
        match rt {
            JokersWild => {
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
        print!("{}", JokersWild);
        println!()
    }

    /// Not directly relevant for Rummikub, but works, and demonstrates how to
    /// create a sorting behavior for a vector of vectors
    /// Since it is more difficult to implement the ord trait for vectors since I don't own that crate
    /// There may be a way to do it with associated types?
    /// see: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
    #[test]
    fn scratch_sorting_vectors_against_vectors() {
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
}
