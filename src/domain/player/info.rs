use std::fmt::Formatter;

/// Information about the player, like name, and or someday maybe
/// difficulty, or ranking etc.
/// Also uses fancy Tuple struct with a str reference and static lifetime
/// essentially making this type barely a wrapper, which indeed it is
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct PlayerInfo(String);

impl std::fmt::Display for PlayerInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player {}", self.0)
    }
}

impl PlayerInfo {
    pub fn of(name: &String) -> PlayerInfo {
        PlayerInfo{0: name.clone()}
    }
}