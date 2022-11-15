use std::cmp::Ordering;
use crate::domain::ScoreValue;
use crate::domain::tiles::Tile;
use crate::game_loop::GameState;

///Player racks can hold any number of tiles (up to all tiles not had by other players)
/// This information is known only to the owning player
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerRack {
    rack: Vec<Tile>,
    played_initial_meld: bool,
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
        true
    }

    pub fn total_value(&self) -> ScoreValue {
        todo!()
    }

}