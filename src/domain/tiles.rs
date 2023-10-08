use crate::domain::score_value::ScoreValue;
use crate::domain::tiles::Number::{
    Eight, Eleven, Five, Four, Nine, One, Seven, Six, Ten, Thirteen, Three, Twelve, Two,
};
use crate::domain::{Decompose, RummikubError};
use colored::{ColoredString, Colorize};
use rand::seq::IteratorRandom;
use std::fmt::{write, Display, Formatter};
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter, EnumString};
use Tile::{JokersWild, RegularTile};

#[derive(EnumIter, EnumCount, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Color {
    Red,
    Blue,
    Orange,
    Black,
}

impl Color {
    pub fn get_rand() -> Color {
        if let Some(col) = Color::iter().choose(&mut rand::thread_rng()) {
            return col;
        }
        Color::Black
    }

    ///There are many ways to "add one" to enums, this is very pedantic, but
    /// also explicit, and avoids any possible issues with conversions of primitives
    pub fn next(&self) -> Self {
        use Color::*;
        match self {
            Red => Blue,
            Blue => Orange,
            Orange => Black,
            Black => Red,
        }
    }
}

/// Represents the ordered numeric values of the regular Rummikub tiles
/// Not using u8's as representation in order to make illegal states unrepresentable
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, EnumIter, EnumCount, Hash, Copy, Clone,
)]
pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
}

impl Number {
    pub fn get_rand() -> Number {
        if let Some(bar) = Number::iter().choose(&mut rand::thread_rng()) {
            return bar;
        }
        Number::One
    }

    ///There are many ways to "add one" to enums, this is very pedantic, but
    /// also explicit, and avoids any possible issues with conversions of primitives
    pub fn next(&self) -> Number {
        use Number::*;
        match self {
            One => Two,
            Two => Three,
            Three => Four,
            Four => Five,
            Five => Six,
            Six => Seven,
            Seven => Eight,
            Eight => Nine,
            Nine => Ten,
            Ten => Eleven,
            Eleven => Twelve,
            Twelve => Thirteen,
            Thirteen => Thirteen, // TODO Potentially Not Obvious Behavior, consider instead return Some(Number)!!!
        }
    }

    ///There are many ways to "minus one" to enums, this is very pedantic, but
    /// also explicit, and avoids any possible issues with conversions of primitives
    pub fn prev(&self) -> Number {
        use Number::*;
        match self {
            One => One, // TODO Potentially Not Obvious Behavior, consider instead return Some(Number)!!!
            Two => One,
            Three => Two,
            Four => Three,
            Five => Four,
            Six => Five,
            Seven => Six,
            Eight => Seven,
            Nine => Eight,
            Ten => Nine,
            Eleven => Ten,
            Twelve => Eleven,
            Thirteen => Twelve,
        }
    }

    pub fn as_value(&self) -> ScoreValue {
        use Number::*;
        let total: u16 = match self {
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Eleven => 11,
            Twelve => 12,
            Thirteen => 13,
        };
        ScoreValue::of(total as u8)
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct ColoredNumber {
    pub color: Color,
    pub num: Number,
}

impl ColoredNumber {
    pub fn new(c: Color, n: Number) -> Self {
        ColoredNumber { color: c, num: n }
    }

    pub fn get_rand() -> Self {
        ColoredNumber {
            color: Color::get_rand(),
            num: Number::get_rand(),
        }
    }

    pub fn next(&self) -> Result<Self, RummikubError> {
        if self.num < Number::Thirteen {
            return Ok(ColoredNumber {
                num: self.num.next(),
                color: self.color,
            });
        }
        Err(RummikubError)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Tile {
    JokersWild,
    RegularTile(ColoredNumber),
}

impl Decompose for Tile {
    /// can a tile decompose into itself? -> YES lol
    fn decompose(&self) -> Vec<Tile> {
        return vec![self.clone()];
    }
}

/// Given tiles returns their corresponding regular CN's. Drops Jokers
pub fn only_colored_nums(tiles: &Vec<Tile>) -> Vec<ColoredNumber> {
    let mut col_nums: Vec<ColoredNumber> = vec![];
    for &tile in tiles {
        match tile {
            RegularTile(cn) => col_nums.push(cn),
            _ => {}
        }
    }
    col_nums
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text: ColoredString;
        match self {
            JokersWild => text = "J".bright_green(),
            RegularTile(cn) => {
                let num_str: String = cn.num.as_value().to_string();
                match cn.color {
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
    pub fn new(c: Color, n: Number) -> Self {
        let cn = ColoredNumber { color: c, num: n };
        RegularTile(cn)
    }

    pub fn any_regular() -> Self {
        let cn = ColoredNumber {
            color: Color::get_rand(),
            num: Number::get_rand(),
        };
        RegularTile(cn)
    }

    pub fn all_unique_numbered() -> Vec<Self> {
        let mut unique: Vec<Tile> = Vec::new();
        for color in Color::iter() {
            for num in Number::iter() {
                let col_num = ColoredNumber { num, color };
                unique.push(Tile::RegularTile(col_num))
            }
        }
        return unique;
    }

    /// For Learning, English version for this terse line
    /// Using the abbreviated from of the exhaustive match expression, check this tile input,
    /// which happens to be a composite enum of a struct.
    /// Then since it matches, execute the following expression (statement? block? -> *terminology*)
    /// since this case for us is just boolean, return true, otherwise which since the match statement
    /// is using the terse form representing ALL OTHER possible states, return false.
    pub fn is_color(&self, color: Color) -> bool {
        if let RegularTile(cn) = self {
            cn.color == color
        } else {
            false
        }
    }
    pub fn is_number(&self, num: Number) -> bool {
        if let RegularTile(cn) = self {
            cn.num == num
        } else {
            false
        }
    }
    pub fn is_joker(&self) -> bool {
        if let JokersWild = self {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tile_tests {
    use super::{Color, ColoredNumber, Number, Tile};
    use crate::domain::tiles::Color::{Blue, Red};
    use crate::domain::tiles::Number::{Eight, Five};
    use colored::Colorize;
    use rand::prelude::IteratorRandom;
    use strum::{EnumCount, IntoEnumIterator};

    #[test]
    fn correct_cardinality() {
        assert_eq!(Color::COUNT, 4);
        assert_eq!(Number::COUNT, 13)
    }

    #[test]
    fn property_confirmation() {
        let some_tile = Tile::RegularTile(ColoredNumber {
            color: Red,
            num: Number::Twelve,
        });
        assert!(some_tile.is_color(Red));
        assert_eq!(some_tile.is_color(Color::Black), false);
        assert!(some_tile.is_number(Number::Twelve));
        assert_eq!(some_tile.is_number(Number::One), false);
        assert_eq!(some_tile.is_joker(), false);
        assert!(Tile::JokersWild.is_joker());
    }

    #[test]
    fn number_ordering() {
        assert!(Number::One < Number::Two);
        assert!(Number::Two < Number::Thirteen);
        let mut prev = Number::One;
        for num in Number::iter() {
            if num == Number::One {
                continue;
            } else {
                assert!(prev < num);
                prev = num;
            }
        }
    }

    #[test]
    fn color_equality() {
        assert_ne!(Red, Color::Black);
        assert_ne!(Blue, Color::Orange);
    }

    #[test]
    fn colored_number_comparisons() {
        let red5 = ColoredNumber::new(Red, Five);
        let blue8 = ColoredNumber::new(Blue, Eight);
        assert_ne!(red5, blue8);
        let alt_red5 = ColoredNumber::new(Red, Five);
        assert_eq!(red5, alt_red5);
    }

    #[test]
    fn many_ways_to_filter() {}

    #[test]
    fn random_tiles() {
        // TODO sometimes seems to hang? => nope, just the test functionality haning, (must be elsewhere?)
        let rt = Tile::any_regular();
        if let Tile::RegularTile(foo) = rt {
            assert!(true)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn all_unique() {
        //Assert that thsi gives all unique regular tiles. i.e. 13x4 -> 52
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
}
