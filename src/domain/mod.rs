use crate::domain::score_value::{ScoreValue, ScoringRule};
use crate::domain::tiles::Tile;
use tiles::tile_sequence::TileSequence;

pub mod player;
pub mod score_value;
pub mod sets;
pub mod table;
pub mod tiles;

// FYI, doing this instead of mod.rs is the 'preferred' convention
// Preferred by who I don't know, but I like the way it's organized with the mod.rs

#[derive(Debug, PartialEq)]
pub struct RummikubError;

const MAX_TILE_COUNT: u8 = 106;

/// Decomposes an abstract group of multiple (or a single) tiles,
/// into the component tiles that constitute the thing that is being decomposed
pub trait Decompose {
    fn decompose(&self) -> Vec<Tile>;

    /// Returns the count of tiles in the Decomposable
    /// There are 106 total tiles in the game which greatly limits the count
    /// If an item can be decomposed, it can be counted.
    /// They can access other methods declared in the trait
    fn count(&self) -> Result<Count, RummikubError> {
        let tiles: TileSequence = self.decompose();
        let length = tiles.len();
        if length > MAX_TILE_COUNT as usize {
            return Err(RummikubError);
        }
        Ok(Count(length.try_into().map_err(|e| RummikubError)?))
    }

    fn score(&self, rule: ScoringRule) -> Result<ScoreValue, RummikubError>;
}

/// Represents count of an unordered collection of tiles, max is 106 as that is all in the game
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Count(pub u8);

#[cfg(test)]
mod domain_test {
    use crate::domain::score_value::{ScoreValue, ScoringRule};
    use crate::domain::tiles::Tile;
    use crate::domain::{Count, Decompose, RummikubError};

    struct TestDummy;

    // This was unnecessary, but is an interesting way to
    // check default implementation of traits
    impl Decompose for TestDummy {
        fn decompose(&self) -> Vec<Tile> {
            vec![Tile::any_regular(); 100] // Cool array constructor
        }

        /// Can have a default implementation, but still override it
        /// According to the rules, jokers score the value of the tile it represents (e.g. for initial meld)
        /// For Points at the end of the game (i.e. just on the rack, it is 30.)
        fn score(&self, rule: ScoringRule) -> Result<ScoreValue, RummikubError> {
            Ok(ScoreValue::of_u16(0))
        }
    }

    #[test]
    fn count_properties_confirmation() {
        let thing = Tile::any_regular();
        let val = thing.count().expect("ONE");
        assert_eq!(Count(1), val);
    }

    /// Wow, the possibilities here
    #[test]
    fn count_must_not_be_to_big() {
        let dummy = TestDummy;
        assert_eq!(Count(100), dummy.count().expect("100"));
        let wait = vec![Tile::any_regular()];
        assert_eq!(Count(1), wait.count().expect("Must be one"));

        assert_eq!(
            Result::Err(RummikubError),
            vec![Tile::any_regular(); 200].count()
        );
    }
}
