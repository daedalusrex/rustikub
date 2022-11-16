use crate::domain::boneyard::Boneyard;
use crate::domain::tiles::Tile;
use crate::domain::ScoreValue;
use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

const INITIAL_TILES: u8 = 14;

///Player racks can hold any number of tiles (up to all tiles not had by other players)
/// This information is known only to the owning player
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerRack {
    rack: Vec<Tile>,
    played_initial_meld: bool,
}

impl Display for PlayerRack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rack has {} tiles", self.rack.len()) // TODO, consider to elaborate
    }
}

impl PartialOrd for PlayerRack {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayerRack {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_value().cmp(&other.total_value())
    }
}

impl PlayerRack {
    pub fn is_empty(&self) -> bool {
        return self.rack.len() == 0;
    }

    pub fn draw_initial_tiles(draw_pile: &Boneyard) -> (Self, Boneyard) {
        let mut rack: Vec<Tile> = vec![];
        // let mut bones: Cow<'_, Boneyard> = Cow::Borrowed(draw_pile); // An alternative feature, lol Cow
        let mut bones = draw_pile.clone();
        for i in 0..INITIAL_TILES {
            let (tile, new_bones) = bones.draw_one();
            // Learning: doing another let here causes shadowing, which is not the desired behavior
            bones = new_bones;
            rack.push(tile);
        }
        let player_rack = PlayerRack {
            rack,
            played_initial_meld: false,
        };
        (player_rack, bones)
    }

    pub fn total_value(&self) -> ScoreValue {
        todo!()
    }
}

#[cfg(test)]
mod basic_tests {
    use crate::domain::boneyard::Boneyard;
    use crate::domain::player_rack::PlayerRack;

    #[test]
    pub fn init_rack_and_give_back_boneyard() {
        let bones = Boneyard::new_game();
        println!("initial boneyard: {}", bones);
        let result = PlayerRack::draw_initial_tiles(&bones);
        let (new_rack, new_bones) = result;
        println!("{}", new_rack);
        println!("{}", new_bones);
    }
}
