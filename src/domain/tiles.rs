use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, EnumCount};
use Tile::{JokersWild, RegularTile};

#[derive(EnumIter,EnumCount, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Color {
    Red,
    Blue,
    Orange,
    Black,
}

/// Represents the ordered numeric values of the regular rummikub tiles
/// Not using u8's as representation in order to make illegal states unrepresentable
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, EnumIter, EnumCount, Hash, Copy, Clone)]
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

#[derive(Debug, Hash, Clone, PartialEq, Eq, Copy)]
pub struct ColoredNumber {
    pub num: Number,
    pub color: Color,
}

#[derive(Debug, Hash, Clone)]
pub enum Tile {
    JokersWild,
    RegularTile(ColoredNumber),
}

impl Tile {
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
    use strum::{EnumCount};

    #[test]
    fn correct_cardinality() {
        assert_eq!(Color::COUNT, 4);
        assert_eq!(Number::COUNT, 13)
    }

    #[test]
    fn property_confirmation() {
        let some_tile = Tile::RegularTile(ColoredNumber{
            color: Color::Red,
            num: Number::Twelve,
        });
        assert!(some_tile.is_color(Color::Red));
        assert_eq!(some_tile.is_color(Color::Black), false);
        assert!(some_tile.is_number(Number::Twelve));
        assert_eq!(some_tile.is_number(Number::One), false);
        assert_eq!(some_tile.is_joker(), false);
        assert!(Tile::JokersWild.is_joker());
    }
}
