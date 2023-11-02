#![allow(dead_code, unused_imports, unused_variables)] //TODO remove during clean up phase

extern crate core;

use game_loop::meta::GameConfig;

mod domain;
mod game_loop;

fn main() {
    use game_loop::state::GameState;
    use game_loop::*;
    println!("Hello There! Welcome to Rustikub!");
    println!("Now Playing A Game With 4 Players");
    let conf = GameConfig { num_players: 4 };
    let game1 = GameState::init_game(conf);
    let result = main_game_loop(game1);
    println!("Game Complete! Result: {}", result);
}
