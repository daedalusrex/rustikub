use std::fmt::{Display, Formatter};

use group::Group;
use run::Run;

use crate::domain::Decompose;
use crate::domain::score_value::ScoreValue;
use crate::domain::tiles::Tile;

pub mod group;
pub mod run;

#[derive(Debug, Clone, PartialEq)]
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


impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tiles = self.decompose();
        write!(f, "|" ).unwrap();
        for t in tiles { write!(f, "{}", t).unwrap() }
        write!(f, "| " )
    }
}

impl Decompose for Set {
    fn decompose(&self) -> Vec<Tile> {
        match self {
            Set::Group(g) => g.decompose(),
            Set::Run(r) => r.decompose(),
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
