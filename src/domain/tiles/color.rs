use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

#[derive(
    sqlx::Type, EnumIter, EnumCount, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone,
)]
#[sqlx(type_name = "color")]
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

#[cfg(test)]
mod color_tests {
    use super::*;
    use strum::EnumCount;

    #[test]
    fn correct_cardinality() {
        assert_eq!(Color::COUNT, 4);
    }
}
