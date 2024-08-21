use crate::domain::player::Player;
use crate::domain::score_value::ScoringRule::OnRack;
use crate::domain::score_value::{ScoreValue, MAX_SCORE_VALUE};
use crate::domain::Decompose;
use std::fmt;
use std::fmt::Formatter;

/// The final outcome for a given game
pub struct GameOutcome {
    pub winner: Player,
    pub loser: Player,
}

impl fmt::Display for GameOutcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Winner is: {}! With Score: {}.\n Highest Score at End Game(THE LOSER) was {} with {} pts",
            self.winner.info,
            self.winner
                .rack
                .score(OnRack)
                .unwrap_or(ScoreValue::of_u16(MAX_SCORE_VALUE)),
            self.loser.info,
            self.loser
                .rack
                .score(OnRack)
                .unwrap_or(ScoreValue::of_u16(MAX_SCORE_VALUE)) // of_u16 is called eagerly
        )
    }
}

/// Information used to control the type of game played (i.e. number of players)
pub struct GameConfig {
    // TODO add some kind of random seed as optional
    pub num_players: u8,
}
