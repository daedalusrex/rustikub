use crate::domain::table::boneyard::Boneyard;
use crate::domain::tiles::{Color, ColoredNumber as CN, ColoredNumber, Tile, only_colored_nums, Number};
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
use crate::domain::tiles::Tile::{JokersWild, RegularTile};

const INITIAL_TILES: u8 = 14;

// TODO Consider derive Copy instead for consistent use of copy semantics
///Player racks can hold any number of tiles (up to all tiles not had by other players)
/// This information is known only to the owning player
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rack {
    pub rack: Vec<Tile>,
    pub played_initial_meld: bool,
}

impl Display for Rack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rack has {} tiles", self.rack.len())
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

impl Decompose for Rack {
    fn decompose(&self) -> Vec<Tile> {
        return self.rack.clone();
    }
}

impl Rack {
    // TODO also return new rack as part of state, and consistency with other design choices
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
        ScoreValue::add_em_up(&self.rack)
    }

    /// Returns all available sets that currently exist on the rack.
    /// Prefers Runs before Groups, so if a tile is needed in a run, it won't be re-used in a possible group
    /// TODO Empty Vec vs Option Vec? also new rack at same time? is a lot?
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
    /// TODO Current Implementation Ignores Jokers
    fn groups_on_rack(&self) -> Vec<Group> {
        let mut groups: Vec<Group> = vec![];

        let mut reg_tiles = only_colored_nums(&self.rack);
        reg_tiles.sort();
        // Power of derive is very cool
        reg_tiles.dedup();
        for num in Number::iter() {
            let only_matching_nums: Vec<Tile> = reg_tiles.iter()
                .filter(|&cn| cn.num == num)
                .map(|cn| RegularTile(cn.clone()))
                .collect();
            if let Some(found) = Group::parse(only_matching_nums) {
                groups.push(found);
            }
        }
        groups
    }

    /// Returns all Runs that are possible to create simultaneously given tiles currently on the rack.
    /// Multiple concurrent possible runs are not returned
    /// TODO Current Implementation Ignores Jokers
    fn runs_on_rack(&self) -> Vec<Run> {
        let mut runs: Vec<Run> = vec![];

        let mut reg_tiles = only_colored_nums(&self.rack);
        reg_tiles.sort();
        reg_tiles.dedup();
        for color in Color::iter() {
            let all_with_color: Vec<Tile> = reg_tiles.iter()
                .filter(|&cn| cn.color == color)
                .map(|cn| RegularTile(cn.clone()))
                .collect();

            // TODO This is likely not a comprehensive way to find all possible ordered subsets -> BUT WHO CARES
            let mut from_left = all_with_color.clone();
            let mut from_right = all_with_color.clone();
            let mut walk_inwards = all_with_color.clone();
            // Tiles can't be used multiple times in different runs (at least in this simple implementation)
            // So if found any run for a particular color, just stop
            let mut found_at_least_one_for_this_color = false;

            while from_right.len() > 0 && !found_at_least_one_for_this_color {
                if let Some(found) = Run::parse(from_right.clone()).ok() {
                    runs.push(found);
                    found_at_least_one_for_this_color = true;
                }
                from_right.pop();
            }

            while from_left.len() > 0 && !found_at_least_one_for_this_color {
                if let Some(found) = Run::parse(from_left.clone()).ok() {
                    runs.push(found);
                    found_at_least_one_for_this_color = true;
                }
                from_left.remove(0);
            }

            let mut left_or_right = false;
            while walk_inwards.len() > 0 && !found_at_least_one_for_this_color {
                // Consider changing this to a closure
                if let Some(found) = Run::parse(walk_inwards.clone()).ok() {
                    runs.push(found);
                    found_at_least_one_for_this_color = true;
                }
                if left_or_right {
                    walk_inwards.remove(0);
                    left_or_right = true;
                } else {
                    walk_inwards.pop();
                    left_or_right = false;
                }
            }
        }
        runs
    }

    /// Removes the given vector of tiles from the rack and returns a new version
    /// This could be a Tile, or a Group, or a Run
    /// An Error Will be returned if any of the requested tiles are not present in the Rack
    /// Relies on Traits!!
    pub fn remove(&self, items: &impl Decompose) -> Result<Self, RummikubError> {
        // TODO this is broken? Or maybe it's caller
        let tiles = items.decompose();
        let mut remaining = self.rack.clone();
        for tile in &tiles {
            if !remaining.contains(tile) {
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
        let mut mut_face_up = face_up.clone();
        let tiles_to_attempt = self.decompose();
        let mut mut_rack = self.clone();
        let mut change_occured = false;

        for attempt in tiles_to_attempt {
            if let Some(place_success) = mut_face_up.place_new_tile(&attempt) {
                mut_face_up = place_success;
                mut_rack = mut_rack.remove(&attempt).unwrap();
                change_occured = true;
            }
        }

        // TODO replace with partial eq derivation on face up
        if change_occured {
            return Some((mut_rack, mut_face_up));
        } else {
            return None;
        }
    }

    // TODO make return new rack for consistent style
    pub fn add_tile_to_rack(&mut self, tile: &Tile) {
        self.rack.push(*tile);
        self.rack.sort();
    }
}

#[cfg(test)]
mod basic_tests {
    use crate::domain::player::rack::Rack;
    use crate::domain::sets::run::Run;
    use crate::domain::table::boneyard::Boneyard;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use crate::domain::tiles::{Color, ColoredNumber as CN, Number, Tile};
    use crate::domain::{Decompose, RummikubError};
    use crate::domain::sets::group::Group;
    use crate::domain::sets::Set;

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

    #[test]
    pub fn correct_detection_of_runs_simple() {
        let basic_run = Run::of(&CN::new(Color::Black, Number::One), 5).unwrap();
        let other_run = Run::of(&CN::new(Color::Blue, Number::Five), 3).unwrap();
        let mut tiles = vec![];
        tiles.append(basic_run.decompose().as_mut());
        tiles.append(other_run.decompose().as_mut());
        let test_rack = Rack { rack: tiles, played_initial_meld: false };
        let found_runs = test_rack.runs_on_rack();
        let (found_sets, not_care_rack) = test_rack.sets_on_rack().unwrap();
        // Implicitly relies on sorting by color -> Shrugs
        assert_eq!(found_runs, vec![other_run.clone(), basic_run.clone()]);
        assert_eq!(found_sets, vec![Set::Run(other_run), Set::Run(basic_run)]);
    }


    #[test]
    fn correct_detection_of_groups_simple() {
        let basic_group = Group::of(&Number::Five, &vec![Color::Black, Color::Blue, Color::Red]).unwrap();
        let other_group = Group::of(&Number::Twelve, &vec![Color::Red, Color::Black, Color::Orange]).unwrap();

        let mut correct_tiles = vec![];
        correct_tiles.append(basic_group.decompose().as_mut());
        correct_tiles.append(other_group.decompose().as_mut());

        let test_rack = Rack { played_initial_meld: false, rack: correct_tiles };
        let found_groups = test_rack.groups_on_rack();
        let (found_sets, not_care_rack) = test_rack.sets_on_rack().unwrap();
        assert_eq!(found_groups, vec![basic_group.clone(), other_group.clone()]);
        assert_eq!(found_sets, vec![Set::Group(basic_group), Set::Group(other_group)]);
    }

    #[test]
    pub fn detection_run_and_group() {
        let basic_run = Run::of(&CN::new(Color::Black, Number::One), 6).unwrap();
        let other_run = Run::of(&CN::new(Color::Blue, Number::Five), 3).unwrap();
        let basic_group = Group::of(&Number::Five, &vec![Color::Black, Color::Blue, Color::Red]).unwrap();
        let other_group = Group::of(&Number::Twelve, &vec![Color::Red, Color::Black, Color::Orange]).unwrap();

        let mut tiles = vec![];
        tiles.append(basic_run.decompose().as_mut());
        tiles.append(other_run.decompose().as_mut());
        tiles.append(basic_group.decompose().as_mut());
        tiles.append(other_group.decompose().as_mut());
        let test_rack = Rack { rack: tiles, played_initial_meld: false };

        let (found_sets, modified_rack) = test_rack.sets_on_rack().unwrap();

        assert!(modified_rack.is_empty());
        assert_eq!(found_sets, vec![Set::Run(other_run), Set::Run(basic_run), Set::Group(basic_group), Set::Group(other_group)]);
    }
}
