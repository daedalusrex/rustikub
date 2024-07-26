use crate::domain::tiles::{Tile, TileSequence};
use std::num::TryFromIntError;
pub mod player;
pub mod score_value;
pub mod sets;
pub mod table;
pub mod tiles;

// FYI, doing this instead of mod.rs is the 'preferred' convention

#[derive(Debug)]
pub struct RummikubError;

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
        let convert: Result<u8, _> = length.try_into();
        Ok(Count(convert.map_err(|e| RummikubError)?))
    }
}

/// Represents count of an unordered collection of tiles, max is 106 as that is all in the game
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Count(u8);

/// Downsides of new type paradigm, and from using Rummikub error
// impl TryFrom<TryFromIntError> for Count {
//     type Error = RummikubError;
//
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         if value > u8::MAX {
//             Err(RummikubError)
//         } else {
//             Ok(Count(value))
//         }
//     }
// }
#[cfg(test)]
mod domain_test {
    use crate::domain::tiles::Tile;
    use crate::domain::{Count, Decompose};

    #[test]
    fn property_confirmation() {
        let thing = Tile::any_regular();
        let val = thing.count().expect("ONE");
        assert_eq!(Count(1), val);
    }
}
