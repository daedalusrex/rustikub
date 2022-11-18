use crate::domain::tiles::Tile::{JokersWild, RegularTile};
use crate::domain::tiles::*;
use rand::prelude::SliceRandom;
use std::fmt::Formatter;
use strum::IntoEnumIterator;

///Starts with 106 tiles (8 sets of tiles 1-13 in four colours (2 of each), and 2 joker tiles)
#[derive(Debug, Clone, PartialEq)]
pub struct Boneyard {
    pub bones: Vec<Tile>,
}

impl std::fmt::Display for Boneyard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Boneyard count: {}", self.bones.len())
    }
}

impl Boneyard {
    pub fn new_game() -> Self {
        let mut tiles = vec![JokersWild, JokersWild]; // tradeoffs vs push push?
        for color in Color::iter() {
            for num in Number::iter() {
                tiles.push(RegularTile(ColoredNumber { color, num }));
                tiles.push(RegularTile(ColoredNumber { color, num }));
            }
        }
        tiles.shuffle(&mut rand::thread_rng());
        Boneyard { bones: tiles }
    }

    /// Removes one tile from the boneyard, and then returns the modified boneyard, and the new tile
    /// Does NOT reshuffle the pile during a given game, to allow for simpler debugging and determinism
    pub fn draw_one(&self) -> (Tile, Boneyard) {
        let mut new_bones = self.bones.clone();
        // TODO, need to handle when all tiles are drawn, make Option<>
        let draw = new_bones.pop().unwrap();
        (draw, Boneyard { bones: new_bones })
    }
}

#[cfg(test)]
mod test_boneyard {

    use super::*;

    use strum::IntoEnumIterator;

    ///106 tiles in the game, including 104 numbered tiles (valued 1 to 13 in four different colors, two copies of each) and two jokers
    #[test]
    fn verify_initial_state() {
        let state = Boneyard::new_game();
        let bones = state.bones; // Butterfly Meme: Is this a reference? Or a copy? -> No! It's a MOVE!
        assert_eq!(bones.len(), 106);

        for c in Color::iter() {
            let count = bones.iter().filter(|t| t.is_color(c)).count();
            assert_eq!(count, 26);
        }
        for i in Number::iter() {
            let count = bones.iter().filter(|t| t.is_number(i)).count();
            assert_eq!(count, 8);
        }
        let jokers = bones.iter().filter(|t| t.is_joker()).count();
        assert_eq!(jokers, 2);

        // Same test, but now, with Closures!
        // Note, the moving and ownership here is bad, but is an excellent example of syntax
        let pred = |tile: &Tile| tile.is_color(Color::Red);
        let count = bones.into_iter().filter(pred).count();
        assert_eq!(count, 26)
    }

    #[test]
    fn draw_one_gives_new_yard() {
        let bones = Boneyard::new_game();
        let (tile, new_bones) = bones.draw_one();
        let old = bones.bones.len();
        let new = new_bones.bones.len();
        assert_ne!(old, new)
    }
}
