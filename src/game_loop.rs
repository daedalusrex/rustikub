use crate::domain::boneyard::Boneyard;
use crate::domain::player_rack::PlayerRack;
use crate::domain::table::Table;
use crate::domain::ScoreValue;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;

// TODO break some of these into separate files.

/// Represents the publicly known state of a single game of rummikub
#[derive(Debug, Clone)]
pub struct PublicGameState {
    boneyard: Boneyard,
    table: Table,
}

/// The entire current state of a single game of Rummikub
#[derive(Debug, Clone)]
pub struct GameState {
    pub face_up: PublicGameState,
    pub players: VecDeque<Player>,
}

/// The final outcome for a given game
pub struct GameResult {
    winner: Player,
    loser: Player,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Winner is: {}! Highest Score at End Game(THE LOSER) was {} with {} pts",
            self.winner.info.name,
            self.loser.info.name,
            self.loser.rack.total_value()
        )
    }
}

/// Information used to control the type of game played (i.e. number of players)
pub struct GameConfig {
    pub num_players: u8,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct PlayerInfo {
    pub name: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Player {
    pub info: PlayerInfo,
    pub rack: PlayerRack,
}

impl GameState {
    /// Initializes game loop based on provided configuration
    pub fn init_game(conf: GameConfig) -> GameState {
        let mut board = PublicGameState {
            boneyard: Boneyard::new_game(),
            table: Table::new(),
        };

        let mut players = VecDeque::new();
        for i in 0..conf.num_players {
            let (rack, new_bones) = PlayerRack::draw_initial_tiles(&board.boneyard);
            board.boneyard = new_bones;
            let info = PlayerInfo{name: format!("Player {}", i)};
            players.push_back(Player{rack, info});
        }
        GameState{face_up: board, players}
    }
}

/// Modifies Potentially the Entire Table, and returns a new game state
/// Cannot Modify Other Player Racks, but can modify itself
pub fn take_turn(rack: &PlayerRack, face_up: &PublicGameState) -> (PlayerRack, PublicGameState) {
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

pub fn main_game_loop(initial_state: GameState) -> GameResult {
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
    GameResult { winner, loser }
}
