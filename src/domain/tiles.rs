use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use Tile::{JokersWild, RegularTile};

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Color {
    Red,
    Blue,
    Orange,
    Black,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, EnumIter, Hash, Copy, Clone)]
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
    //TODO, it's tempting to put u8's here, but for now, I'm not going to because
    // I want to make illegal states unrepresentable.
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
    /// if let expression crunches brain. For learning, English words for this terse line
    /// Using the abbreviated from of the exhaustive match expression, check this tile input,
    /// which happens to be a composite enum, such that only succeed in the condition where
    /// it's color matches our condition (originally blue but now runtime),
    /// Then since it matches, execute the following expression (statement? block? -> *terminology*)
    /// since this case for us is just boolean, return true, otherwise which since the match statement
    /// is using the terse form representing ALL OTHER possible states, return false.
    /// this is just a predicate. however, it can AND SHOULD be an impl behavior of tile! terse?
    /// Also this is apparently local to the test module
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

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn print_some_types() {
        let color: Color = Color::Black;
        assert_eq!(color, Color::Black);
        println!("Dingus {:?}", color);
        let tile: Tile = Tile::RegularTile(ColoredNumber {
            color: Color::Red,
            num: Number::Twelve,
        });
        // Todo iterate throught a list of enums
        println!("How bout a tile: {:?}", tile)
    }
}
