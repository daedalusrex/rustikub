use crate::domain::player::info::PlayerInfo;
use crate::domain::player::rack::Rack;
use crate::domain::player::Player;
use crate::domain::table::boneyard::Boneyard;
use crate::domain::table::face_up::FaceUpTiles;
use crate::domain::tiles::color::Color::*;
use crate::domain::tiles::number::Number;
use crate::domain::tiles::number::Number::*;
use crate::domain::tiles::Tile;
use crate::game_loop::meta::GameConfig;
use std::collections::VecDeque;
use Tile::RegularTile;

/// Represents the publicly known state of a single game of rummikub
#[derive(Debug, Clone)]
pub struct PublicGameState {
    pub boneyard: Boneyard,
    pub face_up: FaceUpTiles,
}

/// The entire current state of a single game of Rummikub
#[derive(Debug, Clone)]
pub struct GameState {
    pub table: PublicGameState,
    pub players: VecDeque<Player>,
}

impl GameState {
    /// Initializes game loop based on provided configuration
    pub fn init_game(conf: GameConfig) -> GameState {
        let mut board = PublicGameState {
            boneyard: Boneyard::new_game(),
            face_up: FaceUpTiles::new(),
        };

        let mut players = VecDeque::new();
        for i in 1..=conf.num_players {
            if i == 1 {
                println!("DEBUGGING OVERRIDE OF PLAYER ONE");
                let special_case: Vec<Tile> = vec![
                    RegularTile(Red, One),
                    RegularTile(Red, Seven),
                    RegularTile(Red, Eleven),
                    RegularTile(Blue, Eight),
                    RegularTile(Orange, One),
                    RegularTile(Orange, Two),
                    RegularTile(Orange, Seven),
                    RegularTile(Orange, Twelve),
                    RegularTile(Orange, Thirteen),
                    RegularTile(Black, Two),
                    RegularTile(Black, Four),
                    RegularTile(Black, Eleven),
                    RegularTile(Black, Twelve),
                    RegularTile(Black, Thirteen),
                    // OMG! could it be that number sorting thing I did? Next Thirteen is always Thirteen? AN INFINITE LOOP?
                ];

                let p1_rack = Rack {
                    rack: special_case,
                    played_initial_meld: false,
                };
                println!("CREATED RACK {}", p1_rack);
                let info = PlayerInfo::of(&i.to_string());
                players.push_back(Player {
                    rack: p1_rack,
                    info,
                })
            } else {
                let (rack, new_bones) = Rack::draw_initial_tiles(&board.boneyard);
                board.boneyard = new_bones;
                let info = PlayerInfo::of(&i.to_string());
                players.push_back(Player { rack, info });
            }
        }
        GameState {
            table: board,
            players,
        }
    }
}
