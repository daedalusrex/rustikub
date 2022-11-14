use crate::domain::tiles::Tile;
use crate::game_loop::GameState;

/// Player has a rack, and Can, DO things, but does it matter this distinction? -> Naw
#[derive(Debug)]
pub struct PlayerRack {
    rack: Vec<Tile>,
    played_initial_meld: bool,
}


impl PlayerRack {
    pub fn is_empty(&self) -> bool {
        true
    }

    /// Modifies Potentially the Entire Table, and returns a new game state
    /// Cannot Modify Other Player Racks, but can modify itslef
    pub fn take_turn(&mut self) -> GameState {
        todo!()
    }


}