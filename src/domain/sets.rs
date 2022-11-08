pub mod group;
pub mod run;

use group::Group;
use run::Run;

pub enum Set {
    // There are two kinds of sets, either a group or a run
    Group(Group),
    Run(Run),
}
