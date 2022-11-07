pub mod run;
pub mod group;

use run::Run;
use group::Group;

pub enum Set {
    // There are two kinds of sets, either a group or a run
    Group(Group),
    Run(Run),
}



