#![allow(dead_code, unused_imports, unused_variables)] //TODO remove during clean up phase

// TODO add logging crate (Tracing), and replace print lines

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

#[cfg(test)]
mod end_to_end_run_game_for_debugging {
    use super::*;
    use crate::game_loop::main_game_loop;
    use crate::game_loop::state::GameState;
    use std::fmt::Display;

    #[test]
    fn run_game() {
        let conf = GameConfig { num_players: 4 };
        let game1 = GameState::init_game(conf);
        let result = main_game_loop(game1);
        let huh = format!("Test {}", result);
        println!("{huh}");
    }
}
