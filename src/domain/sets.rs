pub mod group;
pub mod run;

use group::Group;
use run::Run;

pub enum Set {
    // There are two kinds of sets, either a group or a run
    Group(Group),
    Run(Run),
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
}