#![allow(dead_code, unused_imports, unused_variables)] //TODO remove during clean up phase

extern crate core;

mod domain;
mod game_loop;

fn main() {
    use game_loop::*;
    println!("Hello There! Welcome to Rustikub!");
    // TODO Hardcoded for now, later can be controlled by user
    println!("Now Playing A Game With 4 Players");
    let conf = GameConfig{num_players: 4};
    let game1 = GameState::init_game(conf);
    let end_game = game1.play_game();
}
