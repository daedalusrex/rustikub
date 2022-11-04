use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter, EnumString};
use Tile::{JokersWild, RegularTile};

#[derive(EnumIter, EnumCount, Debug, PartialEq, Eq, Hash, Copy, Clone)]
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
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Copy)]
pub struct ColoredNumber {
    pub num: Number,
    pub color: Color,
}

impl ColoredNumber {
    pub fn new(c: Color, n: Number) -> Self {
        ColoredNumber { color: c, num: n }
    }
}

#[derive(Debug, Hash, Clone)]
pub enum Tile {
    JokersWild,
    RegularTile(ColoredNumber),
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
    fn random_tiles() {
        let rt = Tile::any_regular();
        if let (Tile::RegularTile(foo)) = rt {
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
