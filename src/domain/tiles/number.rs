use crate::domain::score_value::ScoreValue;
use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter, EnumString};

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
        use crate::domain::tiles::Number::{
            Eight, Eleven, Five, Four, Nine, One, Seven, Six, Ten, Thirteen, Three, Twelve, Two,
        };
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
        use crate::domain::tiles::Number::{
            Eight, Eleven, Five, Four, Nine, One, Seven, Six, Ten, Thirteen, Three, Twelve, Two,
        };
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
        use crate::domain::tiles::Number::{
            Eight, Eleven, Five, Four, Nine, One, Seven, Six, Ten, Thirteen, Three, Twelve, Two,
        };
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
