use crate::domain::tiles::Tile;
use crate::domain::RummikubError;
use std::fmt;
use std::fmt::Formatter;
use std::i16::MAX;
use strum_macros::{EnumCount, EnumIter, EnumString};

const JOKER_U16: u16 = 30u16;

// u16, because max theoretical ScoreValue of all tiles (i.e. the Boneyard) is
// ((13*14)/2 * 8 sets + 60 (2 30 pt jokers) = 788, which is > u8::MAX (255)
pub const MAX_SCORE_VALUE: u16 = (13 * 14 / 2) * 8 + 2 * JOKER_U16;

#[derive(Debug, PartialEq, Ord, Eq, PartialOrd, Copy, Clone)]
pub struct ScoreValue(u16);

pub const JOKER_RACK_SCORE: ScoreValue = ScoreValue::of_u16(JOKER_U16);

#[derive(Debug, PartialEq, PartialOrd, Hash, Clone, Copy, Default)]
pub enum ScoringRule {
    #[default]
    OnRack,
    // On Table only makes sense for comparing sets, in certain situations
    OnTable,
}

impl ScoreValue {
    /// A const fn to create a score value at compile time
    pub const fn of_u16(val: u16) -> ScoreValue {
        if val > MAX_SCORE_VALUE {
            // TODO This blows up... something is not right lol
            panic!("Impossibly large score value!");
        }
        ScoreValue(val)
    }

    pub const fn as_u16(self) -> u16 {
        self.0
    }

    /// Creates an arbitrary score value from the provided integer
    /// Takes any type that could be converted to a u16
    /// Nice to have but not essential
    pub fn of<T>(val: T) -> Result<ScoreValue, RummikubError>
    where
        T: TryInto<u16> + Copy,
    {
        let val = val.try_into().map_err(|e| RummikubError)?;
        if val > MAX_SCORE_VALUE {
            return Err(RummikubError);
        }
        Ok(ScoreValue(val))
    }
}

impl fmt::Display for ScoreValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add<ScoreValue> for ScoreValue {
    type Output = ScoreValue;
    fn add(self, rhs: ScoreValue) -> Self::Output {
        ScoreValue(self.0 + rhs.0)
    }
}

impl std::ops::Mul<u16> for ScoreValue {
    type Output = ScoreValue;
    fn mul(self, rhs: u16) -> Self::Output {
        ScoreValue(self.0 * rhs)
    }
}

impl std::ops::AddAssign for ScoreValue {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

#[cfg(test)]
pub mod quicktests {
    use super::*;
    use crate::domain::score_value::{ScoreValue, ScoringRule};
    use crate::domain::tiles::color::Color;
    use crate::domain::tiles::number::Number;
    use crate::domain::Decompose;

    use crate::domain::tiles::Tile::{JokersWild, RegularTile};

    #[test]
    fn quick_test_of_score_syntactic_sugar() {
        let left = ScoreValue(9);
        let right = ScoreValue(20);
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
        assert_eq!(
            tiles.score(ScoringRule::default()).unwrap(),
            ScoreValue::of_u16(36u16)
        )
    }
}
