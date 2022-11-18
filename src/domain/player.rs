use crate::domain::player::info::PlayerInfo;
use crate::domain::player::rack::Rack;

pub mod info;
pub mod initial_meld;
pub mod rack;

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Player {
    pub info: PlayerInfo,
    pub rack: Rack,
}
