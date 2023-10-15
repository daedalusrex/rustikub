use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

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
        use crate::domain::tiles::Color::{Black, Blue, Orange, Red};
        use Color::*;
        match self {
            Red => Blue,
            Blue => Orange,
            Orange => Black,
            Black => Red,
        }
    }
}
