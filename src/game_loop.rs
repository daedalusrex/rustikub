pub mod meta;
pub mod state;

use crate::domain::table::boneyard::Boneyard;

use crate::domain::player::info::PlayerInfo;
use crate::domain::player::rack::Rack;
use crate::domain::player::Player;
use crate::domain::table::face_up::FaceUpTiles;
use crate::domain::score_value::ScoreValue;
use meta::GameOutcome;
use state::{GameState, PublicGameState};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;

/// Modifies Potentially the Entire Table, and returns a new game state
/// Cannot Modify Other Player Racks, but can modify itself
pub fn take_turn(rack: &Rack, face_up: &PublicGameState) -> (Rack, PublicGameState) {
    //simple stuff first

    if !rack.played_initial_meld {
        if let Some(meld) = rack.can_play_initial_meld() {
            // remove meld from rack
        }
    } else {
        // can attempt to add new tiles to the table
    }

    todo!()
}

pub fn main_game_loop(initial_state: GameState) -> GameOutcome {
    let mut current_state = initial_state.clone();
    let mut current_player = current_state.players.pop_front().unwrap();

    while !current_player.rack.is_empty() {
        let (rack, new_face_up) = take_turn(&current_player.rack, &current_state.face_up);
        let updated_player = Player {
            info: current_player.info.clone(),
            rack,
        };
        current_state.players.push_back(updated_player);
        current_state.face_up = new_face_up;
        current_player = current_state.players.pop_front().unwrap();
    }

    //End Game, Compute Result
    let winner = current_player;
    // TODO not sure that ordering worked, test this
    let loser = current_state.players.iter().max().unwrap().clone();
    GameOutcome { winner, loser }
}
