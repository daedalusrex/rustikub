use crate::domain::table::boneyard::Boneyard;
use crate::domain::tiles::{Color, ColoredNumber as CN, ColoredNumber, Tile};
use crate::domain::{Decompose, RummikubError};
use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use strum::IntoEnumIterator;

use crate::domain::player::initial_meld::InitialMeld;
use crate::domain::score_value::ScoreValue;
use crate::domain::sets::group::Group;
use crate::domain::sets::run::Run;
use crate::domain::sets::Set;
use crate::domain::table::face_up::FaceUpTiles;
use crate::domain::tiles::Tile::RegularTile;

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
        if let Some((sets, rack)) = self.sets_on_rack() {
            return InitialMeld::parse(sets);
        }
        None
    }

    pub fn remove_meld(&self, meld: &InitialMeld) -> Result<Rack, RummikubError> {
        let mut rack = self.clone();
        for set in &meld.sets {
            rack = rack.remove(set)?;
        }
        rack.played_initial_meld = true;
        Ok(rack)
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
        rack.sort();
        let player_rack = Rack {
            rack,
            played_initial_meld: false,
        };
        (player_rack, bones)
    }

    pub fn total_value(&self) -> ScoreValue {
        todo!()
    }

    /// Returns all available sets and groups currently exist on the rack.
    /// Prefers Runs before Groups, so if a tile is needed in a run, it won't be re-used in a possible group
    pub fn sets_on_rack(&self) -> Option<(Vec<Set>, Rack)> {
        let mut sets: Vec<Set> = vec![];
        let mut new_rack = self.clone();
        for r in self.runs_on_rack() {
            sets.push(Set::Run(r.clone()));
            new_rack = new_rack
                .remove(&r)
                .expect("Unable to remove set from rack, which claims it exists!");
        }
        let runless_rack = new_rack.clone();
        for g in runless_rack.groups_on_rack() {
            sets.push(Set::Group(g.clone()));
            new_rack = new_rack
                .remove(&g)
                .expect("Unable to remove set from rack, which claims it exists!");
        }
        return if sets.len() == 0 {
            None
        } else {
            Some((sets, new_rack))
        };
    }

    /// Returns all Groups that are possible to create given the tiles currently present on the rack
    fn groups_on_rack(&self) -> Vec<Group> {
        vec![]
        // TODO implement this
    }

    /// Returns all Runs that are possible to create simultaneously given tiles currently on the rack.
    /// Multiple concurrent possible runs are not returned
    fn runs_on_rack(&self) -> Vec<Run> {
        // TODO test and fix this
        let mut runs_present: Vec<Run> = vec![];
        let num_jokers = self.rack.iter().filter(|tile| tile.is_joker()).count();
        let reg_tiles: Vec<Tile> = self
            .rack
            .iter()
            .filter(|tile| !tile.is_joker())
            .map(|t| t.clone()) // TODO this seems like a dumb work around here
            .collect();

        // JFC look at this thing, in the end all it does is just pop out the useful ColoredNumbers
        let reg_cn: Vec<CN> = self
            .rack
            .iter()
            .filter(|tile| !tile.is_joker())
            .map(|reg_tile| match reg_tile {
                RegularTile(cn) => cn.clone(),
                Tile::JokersWild => panic!("Filtering Out of Jokers is Broken"),
            })
            .collect();

        // Search for Runs Color by Color
        for color in Color::iter() {
            let mut only_col: Vec<Tile> = reg_tiles.iter().filter(|t| t.is_color(color)).map(|t| t.clone()).collect();
            only_col.sort(); // Sorts By Number Order (Lowest is First)
            //Remove Tiles from the Front First, Prioritize Higher Value Runs
            let mut from_left = only_col.clone();
            while !from_left.is_empty() {
                let try_run = Run::parse(from_left.clone());
                if try_run.is_ok() {
                    runs_present.push(try_run.unwrap());
                    from_left.clear();
                }
                from_left.remove(0);
            }
            // TODO should also scan from Right, but not use any tiles present from previous runs, (remove from reg tiles)
        }
        runs_present
    }

    /// Removes the given vector of tiles from the rack and returns a new version
    /// This could be a Tile, or a Group, or a Run
    /// An Error Will be returned if any of the requested tiles are not present in the Rack
    /// Relies on Traits!!
    pub fn remove(&self, items: &impl Decompose) -> Result<Self, RummikubError> {
        let tiles = items.decompose();
        let mut remaining = self.rack.clone();
        for tile in &tiles {
            if !self.rack.contains(tile) {
                return Err(RummikubError);
            }
            let pos = remaining.iter().position(|r_tile| r_tile == tile);
            let some_pos = pos.ok_or(RummikubError)?;
            remaining.swap_remove(some_pos);
        }
        remaining.sort();
        Ok(Rack {
            rack: remaining,
            played_initial_meld: self.played_initial_meld,
        })
    }

    /// If possible, places one (or more) tiles from from the rack onto the face up tiles
    /// following the constraints for groups and sets. Returns the new Rack and New Tiles if successful
    /// otherwise returns None, indicating no change was made
    pub fn rearrange_and_place(&self, face_up: &FaceUpTiles) -> Option<(Rack, FaceUpTiles)> {
        // TODO Implement Rearrange and Place from Rack
        None
    }

    pub fn add_tile_to_rack(&mut self, tile: &Tile) {
        // TODO should be easy, probably worth a test or two. 
        todo!()
    }
}

#[cfg(test)]
mod basic_tests {
    use crate::domain::player::rack::Rack;
    use crate::domain::sets::run::Run;
    use crate::domain::table::boneyard::Boneyard;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use crate::domain::tiles::{Color, ColoredNumber as CN, Number, Tile};
    use crate::domain::RummikubError;

    fn object_mother_some_rack() -> Rack {
        Rack {
            rack: vec![
                JokersWild,
                Tile::any_regular(),
                Tile::any_regular(),
                Tile::any_regular(),
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
        let simple_run_vec = vec![
            RegularTile(CN::new(Color::Black, Number::Five)),
            RegularTile(CN::new(Color::Black, Number::Six)),
            RegularTile(CN::new(Color::Black, Number::Seven)),
        ];
        let simple_run = Run::parse(simple_run_vec.clone()).unwrap();
        let some_rack = object_mother_some_rack();
        let result: Result<Rack, RummikubError> = some_rack.remove(&simple_run);
        assert!(result.is_ok());
    }
}
