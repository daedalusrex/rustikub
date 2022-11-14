use crate::domain::boneyard::Boneyard;
use crate::domain::player_rack::PlayerRack;
use crate::domain::table::Table;

/// Represents the entire state of a single game of rummikub
pub struct GameState {
    boneyard: Boneyard,
    table: Table,
    // TODO maybe split into shareable state, and player owned states
    // TODO also need to separate GameState and TurnManagement/Functionality
    players: Vec<PlayerRack>,
}
/// The final outcome for a given game
pub struct GameResult;
/// Information used to control the type of game played (i.e. number of players)
pub struct GameConfig {
    pub num_players: u8
}

impl GameState {
    /// Initializes game loop based on provided configuration
    pub fn init_game(conf: GameConfig) -> GameState {
        todo!()
    }

    /// Entry function for a single game of rummikub
    pub fn play_game(&self) -> GameResult {
        GameResult
    }
}