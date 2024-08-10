use crate::domain::tiles::Tile;
use std::fmt;
use std::fmt::Formatter;

// u16, because max theoretical ScoreValue of all tiles (i.e. the Boneyard) is
// ((13*14)/2 * 4 + 60 = 424, which is > u8::MAX (255)

#[derive(Debug, PartialEq, Ord, Eq, PartialOrd, Copy, Clone)]
pub struct ScoreValue {
    total: u16,
}

impl ScoreValue {
    /// Creates an arbitrary score value from the provided integer
    /// TODO add parameter for handling scoring on rack vs on the table
    ///     need that parameter, because score evaluation of state rules are different if joker on rack
    pub const fn of(val: u16) -> ScoreValue {
        ScoreValue { total: val }
    }

    /// Adds a sequence of tiles for their face value. In this abstract case, Jokers are worth 30 pts, as they
    /// are not attached to any Set.
    pub fn add_em_up(tiles: &Vec<Tile>) -> ScoreValue {
        let mut score = ScoreValue::of(0);
        for tile in tiles {
            match tile {
                Tile::RegularTile(c, n) => score += n.as_value(),
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

impl std::ops::Mul<u16> for ScoreValue {
    type Output = ScoreValue;
    fn mul(self, rhs: u16) -> Self::Output {
        let total = self.total * rhs;
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
    use crate::domain::tiles::color::Color;
    use crate::domain::tiles::number::Number;

    use crate::domain::tiles::Tile::{JokersWild, RegularTile};

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
            RegularTile(Color::Red, Number::One),
            RegularTile(Color::Blue, Number::Two),
            RegularTile(Color::Black, Number::Three),
            JokersWild,
        ];
        assert_eq!(ScoreValue::add_em_up(&tiles), ScoreValue::of(36))
    }
}
