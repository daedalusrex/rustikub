#[cfg(test)]
mod end_to_end_run_game_for_debugging {

    // Doesn't work because not set up as library?
    // https://www.reddit.com/r/rust/comments/ksenfh/unable_to_import_crate_into_integration_tests/
    // use crate::rustikub::

    #[test]
    fn run_game() {
        // let conf = GameConfig { num_players: 4 };
        // let game1 = GameState::init_game(conf);
        // let result = main_game_loop(game1);
    }

    #[test]
    fn walking_skeleton() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
