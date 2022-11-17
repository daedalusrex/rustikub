pub mod group;
pub mod run;

use group::Group;
use run::Run;
use crate::domain::ScoreValue;

#[derive(Debug, Clone)]
pub enum Set {
    // There are two kinds of sets, either a group or a run
    Group(Group),
    Run(Run),
}

impl Set {
    pub fn total_value(&self) -> ScoreValue {
        match self {
            Set::Group(g) => g.total_value(),
            Set::Run(r) => r.total_value(),
        }
    }
}

/// Certain types of erros that can occur when attempting to parse a collection of tiles
/// into a particular type of Set
#[derive(PartialEq, Debug)]
pub enum ParseError {
    TooManyTiles,
    TooFewTiles,
    DuplicateColors,
    DistinctColors,
    DuplicateNumbers,
    OutOfOrder,
    OutOfBounds,
    IllegalJokers,
}
