use crate::domain::score_value::ScoreValue;
use crate::domain::score_value::ScoringRule::OnRack;
use crate::domain::sets::Set;
use crate::domain::Decompose;

const MINIMUM_MELD_SCORE: ScoreValue = ScoreValue::of_u16(30u16);

/// In order to make an initial meld, each player must place tiles in one or more sets that total at least 30 points.
/// These points must come from the tiles on each player’s rack only.
/// For their initial meld, players may not use tiles already played on the table.
/// A joker used in the initial meld scores the value of the tile it represents.
/// After a player has made their initial meld, they can build on other sets on the table with tiles from their rack
pub struct InitialMeld {
    pub sets: Vec<Set>,
}

impl InitialMeld {
    pub fn parse(candidates: Vec<Set>) -> Option<InitialMeld> {
        let mut score_sum = ScoreValue::of_u16(0u16);
        for set in &candidates {
            match set {
                Set::Group(g) => score_sum += g.score(OnRack).ok()?,
                Set::Run(r) => score_sum += r.score(OnRack).ok()?,
            }
        }
        if score_sum > MINIMUM_MELD_SCORE {
            return Some(InitialMeld {
                sets: candidates.clone(),
            });
        }
        None
    }
}
