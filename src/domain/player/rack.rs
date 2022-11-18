use crate::domain::table::boneyard::Boneyard;
use crate::domain::tiles::Tile;
use crate::domain::{Decompose, RummikubError};
use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::domain::player::initial_meld::InitialMeld;
use crate::domain::score_value::ScoreValue;
use crate::domain::sets::group::Group;
use crate::domain::sets::run::Run;
use crate::domain::sets::Set;

const INITIAL_TILES: u8 = 14;

pub struct GenericT<T>(T);

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

    pub fn sets_on_rack(&self) -> Option<Vec<Set>> {
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

    /// Removes the given vector of tiles from the rack and returns a new version
    /// This could be a Tile, or a Group, or a Run
    /// An Error Will be returned if any of the requested tiles are not present in the Rack
    /// Relies on Traits!!
    pub fn remove(&self, items: &impl Decompose) -> Result<Self, RummikubError> {
        let foo = items.decompose();
        println!("{:?}", foo);
        todo!()
    }

    pub fn testgenericsyntax<T>(&self, foo: GenericT<T>) -> Self {
        todo!()
    }
}


#[cfg(test)]
mod basic_tests {
    use crate::domain::player::rack::Rack;
    use crate::domain::RummikubError;
    use crate::domain::sets::run::Run;
    use crate::domain::table::boneyard::Boneyard;
    use crate::domain::tiles::{Color, ColoredNumber as CN, Number, Tile};
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};

    fn object_mother_some_rack() -> Rack {
        Rack {
            rack: vec![JokersWild,
                       Tile::any_regular(), Tile::any_regular(), Tile::any_regular(),
                       RegularTile(CN::new(Color::Black, Number::Five)),
                       RegularTile(CN::new(Color::Black, Number::Six)),
                       RegularTile(CN::new(Color::Black, Number::Seven)),
            ],
            played_initial_meld: false,
        }
    }

    #[test]
    pub fn init_rack_and_give_back_boneyard() {
        let bones = Boneyard::new_game();
        println!("initial boneyard: {}", bones);
        let result = Rack::draw_initial_tiles(&bones);
        let (new_rack, new_bones) = result;
        println!("{}", new_rack);
        println!("{}", new_bones);
    }

    #[test]
    pub fn removal_of_different_items() {
        let simple_run_vec = vec![RegularTile(CN::new(Color::Black, Number::Five)),
                              RegularTile(CN::new(Color::Black, Number::Six)),
                              RegularTile(CN::new(Color::Black, Number::Seven))];
        let simple_run = Run::parse(simple_run_vec).unwrap();
        let some_rack = object_mother_some_rack();
        let result: Result<Rack, RummikubError> = some_rack.remove(&simple_run);
        assert!(result.is_ok());
    }
}
