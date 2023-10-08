use crate::domain::tiles::Tile;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Ord, Eq, PartialOrd, Copy, Clone)]
pub struct ScoreValue {
    total: u16,
}

impl ScoreValue {
    /// Creates an arbitrary score value from the provided integer
    pub const fn of(val: u8) -> ScoreValue {
        ScoreValue { total: val as u16 }
    }

    /// Adds a sequence of tiles for their face value. In this abstract case, Jokers are worth 30 pts, as they
    /// are not attached to any Set.
    pub fn add_em_up(tiles: &Vec<Tile>) -> ScoreValue {
        let mut score = ScoreValue::of(0);
        for tile in tiles {
            match tile {
                Tile::RegularTile(cn) => score += cn.num.as_value(),
                Tile::JokersWild => score += ScoreValue::of(30),
            }
        }
        score
    }
}

impl fmt::Display for ScoreValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.total)
    }
}

impl std::ops::Add<ScoreValue> for ScoreValue {
    type Output = ScoreValue;
    fn add(self, rhs: ScoreValue) -> Self::Output {
        let new_val = self.total + rhs.total;
        ScoreValue { total: new_val }
    }
}

impl std::ops::Mul<u8> for ScoreValue {
    type Output = ScoreValue;
    fn mul(self, rhs: u8) -> Self::Output {
        let total = self.total * rhs as u16;
        ScoreValue { total }
    }
}

impl std::ops::AddAssign for ScoreValue {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            total: self.total + rhs.total,
        };
    }
}

#[cfg(test)]
pub mod quicktests {
    use crate::domain::score_value::ScoreValue;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use crate::domain::tiles::{Color, ColoredNumber, Number};

    #[test]
    fn quick_test_of_score_syntactic_sugar() {
        let left = ScoreValue { total: 9 };
        let right = ScoreValue { total: 20 };
        let mut sum = left + right;
        println!("sum: {:?}", &sum);
        sum += left;
        println!("plus eq {:?}", sum);
    }

    #[test]
    fn score_adding() {
        let tiles = vec![
            RegularTile(ColoredNumber {
                color: Color::Red,
                num: Number::One,
            }),
            RegularTile(ColoredNumber {
                color: Color::Blue,
                num: Number::Two,
            }),
            RegularTile(ColoredNumber {
                color: Color::Black,
                num: Number::Three,
            }),
            JokersWild,
        ];
        assert_eq!(ScoreValue::add_em_up(&tiles), ScoreValue::of(36))
    }
}
