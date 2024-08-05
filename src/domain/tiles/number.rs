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
    /// Does not return Just Number, because that DID lead to an infinite loop in logic
    pub fn next(&self) -> Option<Number> {
        use Number::*;
        match self {
            One => Some(Two),
            Two => Some(Three),
            Three => Some(Four),
            Four => Some(Five),
            Five => Some(Six),
            Six => Some(Seven),
            Seven => Some(Eight),
            Eight => Some(Nine),
            Nine => Some(Ten),
            Ten => Some(Eleven),
            Eleven => Some(Twelve),
            Twelve => Some(Thirteen),
            Thirteen => None,
        }
    }

    ///There are many ways to "minus one" to enums, this is very pedantic, but
    /// also explicit, and avoids any possible issues with conversions of primitives
    pub fn prev(&self) -> Option<Number> {
        use Number::*;
        match self {
            One => None,
            Two => Some(One),
            Three => Some(Two),
            Four => Some(Three),
            Five => Some(Four),
            Six => Some(Five),
            Seven => Some(Six),
            Eight => Some(Seven),
            Nine => Some(Eight),
            Ten => Some(Nine),
            Eleven => Some(Ten),
            Twelve => Some(Eleven),
            Thirteen => Some(Twelve),
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
        ScoreValue::of(total)
    }
}

#[cfg(test)]
mod number_tests {
    use super::*;
    use crate::domain::tiles::number::Number::*;
    use strum::EnumCount;

    #[test]
    fn correct_cardinality() {
        assert_eq!(Number::COUNT, 13)
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
}
