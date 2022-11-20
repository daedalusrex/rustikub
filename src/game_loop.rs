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
pub fn take_turn(rack: &Rack, table: &PublicGameState) -> (Rack, PublicGameState) {
    let mut new_rack = rack.clone();
    let mut new_table = table.clone();
    let mut has_placed_this_turn = false;

    if !rack.played_initial_meld {
        if let Some(meld) = rack.can_play_initial_meld() {
            println!("Playing Initial Meld!");
            // remove meld from rack, must succeed
            new_rack = rack.remove_meld(&meld).unwrap();
            new_table.face_up = table.face_up.place_new_sets(&meld.sets);
            has_placed_this_turn = true;
        }
    }

    if rack.played_initial_meld || new_rack.played_initial_meld {
        // can attempt to add new tiles to the table
        if let Some((new_rack, new_face_up)) = rack.rearrange_and_place(&table.face_up) {
            println!("Rearranged Face Up Tiles and Placing some from Rack!");
            // TODO verify new_rack properly shadowed as expected here
            new_table.face_up = new_face_up;
            has_placed_this_turn = true;
        }
    }

    if !has_placed_this_turn {
        // Have Not Placed Any Tiles This Turn, therefore MUST draw
        println!("Must Draw from Boneyard!");
        let (drawn, new_bones) = table.boneyard.draw_one();
        new_rack.add_tile_to_rack(&drawn);
        new_table.boneyard = new_bones;
    }

    (new_rack, new_table)
}

pub fn main_game_loop(initial_state: GameState) -> GameOutcome {
    let mut current_state = initial_state.clone();
    let mut current_player = current_state.players.pop_front().unwrap();

    while !current_player.rack.is_empty() {
        println!("{}'s Turn!", current_player.info);
        let (rack, table) = take_turn(&current_player.rack, &current_state.table);
        let updated_player = Player {
            info: current_player.info.clone(),
            rack,
        };
        if updated_player.rack.is_empty() {
            current_player = updated_player;
            break;
        }
        current_state.players.push_back(updated_player);
        current_state.table = table;
        current_player = current_state.players.pop_front().unwrap();
    }

    //End Game, Compute Result
    let winner = current_player;
    println!("Game Over! {} Wins!", winner.info);

    // TODO not sure that ordering worked, test this
    let loser = current_state.players.iter().max().unwrap().clone();
    GameOutcome { winner, loser }
}
