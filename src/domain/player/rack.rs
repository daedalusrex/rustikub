use crate::domain::table::boneyard::Boneyard;
use crate::domain::tiles::Tile;
use crate::domain::ScoreValue;
use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::domain::player::initial_meld::InitialMeld;
use crate::domain::sets::group::Group;
use crate::domain::sets::run::Run;
use crate::domain::sets::Set;

const INITIAL_TILES: u8 = 14;

///Player racks can hold any number of tiles (up to all tiles not had by other players)
/// This information is known only to the owning player
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rack {
    pub rack: Vec<Tile>,
    pub played_initial_meld: bool,
}

impl Display for Rack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rack has {} tiles", self.rack.len()) // TODO, consider to elaborate
    }
}

impl PartialOrd for Rack {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rack {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_value().cmp(&other.total_value())
    }
}

impl Rack {
    pub fn can_play_initial_meld(&self) -> Option<InitialMeld> {
        if let Some(sets) = self.sets_on_rack() {
            return InitialMeld::parse(sets);
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        return self.rack.len() == 0;
    }

    pub fn draw_initial_tiles(draw_pile: &Boneyard) -> (Self, Boneyard) {
        let mut rack: Vec<Tile> = vec![];
        // let mut bones: Cow<'_, Boneyard> = Cow::Borrowed(draw_pile); // An alternative feature, lol Cow
        let mut bones = draw_pile.clone();
        for i in 0..INITIAL_TILES {
            let (tile, new_bones) = bones.draw_one();
            // Learning: doing another let here causes shadowing, which is not the desired behavior
            bones = new_bones;
            rack.push(tile);
        }
        let player_rack = Rack {
            rack,
            played_initial_meld: false,
        };
        (player_rack, bones)
    }

    pub fn total_value(&self) -> ScoreValue {
        todo!()
    }

    fn sets_on_rack(&self) -> Option<Vec<Set>> {
        let mut sets: Vec<Set> = vec![];
        for r in self.runs_on_rack() {
            sets.push(Set::Run(r));
        }
        for g in self.groups_on_rack() {
            sets.push(Set::Group(g))
        }
        return if sets.len() == 0 { None } else { Some(sets) };
    }

    fn groups_on_rack(&self) -> Vec<Group> {
        todo!()
    }

    fn runs_on_rack(&self) -> Vec<Run> {
        todo!()
    }
}

#[cfg(test)]
mod basic_tests {
    use crate::domain::player::rack::Rack;
    use crate::domain::table::boneyard::Boneyard;

    #[test]
    pub fn init_rack_and_give_back_boneyard() {
        let bones = Boneyard::new_game();
        println!("initial boneyard: {}", bones);
        let result = Rack::draw_initial_tiles(&bones);
        let (new_rack, new_bones) = result;
        println!("{}", new_rack);
        println!("{}", new_bones);
    }
}
