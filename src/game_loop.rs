pub mod meta;
pub mod state;

use crate::domain::table::boneyard::Boneyard;

use crate::domain::player::info::PlayerInfo;
use crate::domain::player::rack::Rack;
use crate::domain::player::Player;
use crate::domain::score_value::ScoreValue;
use crate::domain::table::face_up::FaceUpTiles;
use meta::GameOutcome;
use state::{GameState, PublicGameState};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;

/// Modifies Potentially the Entire Table, and returns a new game state
/// Cannot Modify Other Player Racks, but can modify itself
pub fn take_turn(prev_rack: &Rack, prev_table: &PublicGameState) -> (Rack, PublicGameState) {
    // TODO theres an infinite loop somewhere in here.
    let mut mut_rack = prev_rack.clone(); // or let mut rack = rack.clone?
    let mut mut_table = prev_table.clone();
    let mut placed_this_turn = false;

    if !mut_rack.played_initial_meld {
        if let Some(meld) = mut_rack.can_play_initial_meld() {
            println!("Playing Initial Meld!");
            // TODO remove meld from rack, must succeed -> Therefore it should not occur here and be part of return
            mut_rack = mut_rack.remove_meld(&meld).unwrap();
            mut_table.face_up = mut_table.face_up.place_new_sets(&meld.sets);
            placed_this_turn = true;
            println!("Table Now Has:\n{}", mut_table.face_up)
        }
    }

    if mut_rack.played_initial_meld {
        // can attempt to add new tiles to the table
        if let Some((complete_sets, rack_without_sets)) = mut_rack.sets_on_rack() {
            println!("Placing Complete Sets from Rack!");
            mut_table.face_up = mut_table.face_up.place_new_sets(&complete_sets);
            mut_rack = rack_without_sets;
            placed_this_turn = true;
            println!("Table Now Has:\n{}", mut_table.face_up)
        }

        if let Some((rack_after_placing, new_face_up)) =
            mut_rack.rearrange_and_place(&mut_table.face_up)
        {
            println!("Rearranged Face Up Tiles and Placing some from Rack!");
            mut_table.face_up = new_face_up;
            mut_rack = rack_after_placing;
            placed_this_turn = true;
            println!("Table Now Has:\n{}", mut_table.face_up)
        }
    }

    if !placed_this_turn {
        // Have Not Placed Any Tiles This Turn, therefore MUST draw
        println!("Must Draw from Boneyard!");

        if let Some((drawn, new_bones)) = prev_table.boneyard.draw_one() {
            mut_rack.add_tile_to_rack(&drawn);
            mut_table.boneyard = new_bones;
        } else {
            //TODO technically this should not happen, but can if players do not play well or hold on forever
            println!("All Tiles have been Drawn! Game Over!");
            // TODO again, taking a shortcut here
            mut_rack = Rack {
                rack: vec![],
                played_initial_meld: true,
            }
        }
    }

    (mut_rack, mut_table)
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
