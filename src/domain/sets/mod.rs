use std::fmt::{format, Display, Formatter};

use group::Group;
use run::Run;

use crate::domain::score_value::{ScoreValue, ScoringRule};
use crate::domain::tiles::Tile;
use crate::domain::{Decompose, RummikubError};

pub mod group;
pub mod run;

#[derive(Debug, Clone, PartialEq)]
pub enum Set {
    // There are two kinds of sets, either a group or a run
    Group(Group),
    Run(Run),
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tiles = self.decompose();
        let whoopsie = format!("EXPLODED writing: {:?}", tiles);
        write!(f, "|").expect("Literally unfailable");
        for t in tiles {
            let this_guy = format!(" This Guy: {:?}", t);
            let msg = whoopsie.clone() + &this_guy;
            write!(f, "{}", t).expect(&*msg);
        }
        write!(f, "| ")
    }
}

impl Decompose for Set {
    fn decompose(&self) -> Vec<Tile> {
        match self {
            Set::Group(g) => g.decompose(),
            Set::Run(r) => r.decompose(),
        }
    }

    fn score(&self, rule: ScoringRule) -> Result<ScoreValue, RummikubError> {
        match self {
            Set::Group(g) => g.score(rule),
            Set::Run(r) => r.score(rule),
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
