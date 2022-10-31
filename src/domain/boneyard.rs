use super::tiles::Tile::{JokersWild, RegularTile};
use super::tiles::*; // TODO use * bad.
use strum::IntoEnumIterator;

// #[derive(PartialEq)] // TODO interesting behavior of Vectors
#[derive(Debug)]
pub struct Boneyard {
    // TODO, new constructor here is important. Perhaps a map? instead?
    pub bones: Vec<Tile>,
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
        Boneyard { bones: tiles }
    }

    pub fn draw_one() -> (Tile, Boneyard) {
        // TODO here, but with the idea of immutablity, when drawing, we get a whole new boneyard
        (
            RegularTile(ColoredNumber {
                color: Color::Red,
                num: Number::Twelve,
            }),
            Boneyard { bones: Vec::new() },
        )
    }
}

#[cfg(test)]
mod test_boneyard {

    use super::*;

    use strum::IntoEnumIterator;

    ///106 tiles (8 sets of tiles 1-13 in four colours (2 of each), and 2 joker tiles)
    /// //106 tiles in the game, including 104 numbered tiles (valued 1 to 13 in four different colors, two copies of each) and two jokers
    #[test]
    fn verify_initial_state() {
        let mut state = Boneyard::new_game();
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
    }
}
