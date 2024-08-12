use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use strum::IntoEnumIterator;

use crate::domain::player::initial_meld::InitialMeld;
use crate::domain::score_value::ScoreValue;
use crate::domain::sets::group::Group;
use crate::domain::sets::run::Run;
use crate::domain::sets::Set;
use crate::domain::table::boneyard::Boneyard;
use crate::domain::tiles::number::Number;
use crate::domain::tiles::tile_sequence::{only_regular_tiles, TileSequence, TileSequenceType};
use crate::domain::tiles::Tile;
use crate::domain::{Decompose, RummikubError};

const INITIAL_TILES: u8 = 14;

/// Player racks can hold any number of tiles (up to all tiles not had by other players)
/// This information is known only to the owning player
/// Cannot derive copy trait because Vec uses heap memory which prevents bitwise copy
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rack {
    pub rack: Vec<Tile>, // TODO make this private, create a nicer constructor
    pub played_initial_meld: bool,
}

impl Display for Rack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ").unwrap();
        for t in &self.rack {
            write!(f, "{}", t).unwrap()
        }
        write!(f, "] ({})", self.rack.len())
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
        self.rack.clone()
    }
}

impl Rack {
    pub fn new(tiles: &TileSequence) -> Result<Rack, RummikubError> {
        tiles.count()?;
        Ok(Rack {
            rack: tiles.clone(),
            played_initial_meld: false,
        })
    }

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
            // Unwrap because CANNOT be empty at start of play
            let (tile, new_bones) = bones.draw_one().unwrap();
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
    /// TODO Hey, couldn't you just mix the rack and the table and perform this same algorithm? (kinda)
    pub fn sets_on_rack(&self) -> Option<(Vec<Set>, Rack)> {
        let mut sets: Vec<Set> = vec![];
        let mut new_rack = self.clone();

        let mut optional_run = self.get_largest_run();
        while let Some(ref largest_run) = optional_run {
            new_rack = new_rack
                .remove(largest_run)
                .expect("Must be able to remove the found run");
            sets.push(Set::Run(largest_run.clone()));
            optional_run = new_rack.get_largest_run();
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
    pub fn groups_on_rack(&self) -> Vec<Group> {
        let mut remaining = TileSequenceType::of(self);
        let mut groups: Vec<Group> = vec![];

        let mut optional_group = remaining.largest_group();
        while let Some(ref largest_group) = optional_group {
            groups.push(largest_group.clone());
            remaining = remaining
                .remove(largest_group)
                .expect("Must be able to remove the found group");
            optional_group = remaining.largest_group();
        }
        groups
    }

    /// Returns the run with the largest score value on the rack if it exists.
    pub fn get_largest_run(&self) -> Option<Run> {
        TileSequenceType::of(self).largest_run()
    }

    /// Removes the given vector of tiles from the rack and returns a new version
    /// This could be a Tile, or a Group, or a Run
    /// An Error Will be returned if any of the requested tiles are not present in the Rack
    /// Relies on Traits!!
    pub fn remove(&self, item: &impl Decompose) -> Result<Self, RummikubError> {
        let rack_tiles = TileSequenceType::of(self);
        let tiles_to_be_removed = item.decompose();
        let mut remaining = rack_tiles.remove(item).ok_or(RummikubError)?;
        remaining.0.sort();
        Ok(Rack {
            rack: remaining.0,
            played_initial_meld: self.played_initial_meld,
        })
    }

    // TODO Remove this, after basic rearrange picks up functionality of just add one
    // pub fn rearrange_and_place(&self, face_up: &FaceUpTiles) -> Option<(Rack, FaceUpTiles)> {
    //     let mut mut_face_up = face_up.clone();
    //     let tiles_to_attempt = self.decompose();
    //     let mut mut_rack = self.clone();
    //     let mut change_occurred = false;
    //
    //     for attempt in tiles_to_attempt {
    //         if let Some(place_success) = mut_face_up.place_new_tile(&attempt) {
    //             mut_face_up = place_success;
    //             mut_rack = mut_rack.remove(&attempt).unwrap();
    //             change_occurred = true;
    //         }
    //     }
    // //     return if change_occurred {
    //         Some((mut_rack, mut_face_up))
    //     } else {
    //         None
    //     };
    // }

    // TODO make return new rack for consistent style
    pub fn add_tile_to_rack(&mut self, tile: &Tile) {
        self.rack.push(*tile);
        self.rack.sort();
    }
}

#[cfg(test)]
mod basic_tests {
    use crate::domain::player::rack::Rack;
    use crate::domain::score_value::ScoreValue;
    use crate::domain::sets::group::Group;
    use crate::domain::sets::run::Run;
    use crate::domain::sets::Set;
    use crate::domain::table::boneyard::Boneyard;
    use crate::domain::tiles::color::Color::*;
    use crate::domain::tiles::number::Number::*;
    use crate::domain::tiles::Tile;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use crate::domain::{Count, Decompose, RummikubError};
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    fn object_mother_some_rack() -> Rack {
        Rack {
            rack: vec![
                JokersWild,
                Tile::any_regular(),
                Tile::any_regular(),
                Tile::any_regular(),
                RegularTile(Black, Five),
                RegularTile(Black, Six),
                RegularTile(Black, Seven),
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
            RegularTile(Black, Five),
            RegularTile(Black, Six),
            RegularTile(Black, Seven),
        ];
        let simple_run = Run::parse(&simple_run_vec).unwrap();
        let some_rack = object_mother_some_rack();
        let result: Result<Rack, RummikubError> = some_rack.remove(&simple_run);
        assert!(result.is_ok());
    }

    #[test]
    pub fn correct_detection_of_runs_simple() {
        let basic_run = Run::of(One, Black, 5).expect("BROKEN");
        let other_run = Run::of(Ten, Blue, 3).expect("BROKEN");
        let mut tiles = vec![];
        tiles.append(basic_run.decompose().as_mut());
        tiles.append(other_run.decompose().as_mut());
        let test_rack = Rack {
            rack: tiles,
            played_initial_meld: false,
        };
        let found_runs = test_rack.get_largest_run().expect("Should have found run");
        let (found_sets, not_care_rack) = test_rack.sets_on_rack().unwrap();
        assert_eq!(found_runs, other_run.clone());
        assert_eq!(found_sets, vec![Set::Run(other_run), Set::Run(basic_run)]);
    }

    #[test]
    fn correct_detection_of_groups_simple() {
        let basic_group = Group::of(Five, &vec![Black, Blue, Red]).unwrap();
        let other_group = Group::of(Twelve, &vec![Red, Black, Orange]).unwrap();

        let mut correct_tiles = vec![];
        correct_tiles.append(basic_group.decompose().as_mut());
        correct_tiles.append(other_group.decompose().as_mut());

        let test_rack = Rack {
            played_initial_meld: false,
            rack: correct_tiles,
        };
        let found_groups = test_rack.groups_on_rack();
        let (found_sets, not_care_rack) = test_rack.sets_on_rack().unwrap();
        assert_eq!(found_groups, vec![other_group.clone(), basic_group.clone()]);
        assert_eq!(
            found_sets,
            vec![Set::Group(other_group), Set::Group(basic_group)]
        );
    }

    #[test]
    pub fn detection_run_and_group() {
        let one_to_six_black = Run::of(One, Black, 6).unwrap();
        let five_to_eight_blue = Run::of(Five, Blue, 3).unwrap();
        let basic_group = Group::of(Five, &vec![Black, Blue, Red]).unwrap();
        let other_group = Group::of(Twelve, &vec![Red, Black, Orange]).unwrap();

        let mut tiles = vec![];
        tiles.append(one_to_six_black.decompose().as_mut());
        tiles.append(five_to_eight_blue.decompose().as_mut());
        tiles.append(basic_group.decompose().as_mut());
        tiles.append(other_group.decompose().as_mut());
        let test_rack = Rack {
            rack: tiles,
            played_initial_meld: false,
        };

        let (found_sets, modified_rack) = test_rack.sets_on_rack().unwrap();

        assert!(modified_rack.is_empty());
        assert_eq!(
            found_sets,
            vec![
                Set::Run(one_to_six_black),
                Set::Run(five_to_eight_blue),
                Set::Group(other_group),
                Set::Group(basic_group),
            ]
        );
    }

    /// The score value of the rack is the value at the end of the game
    /// i.e. Jokers have a value of 30 pts, since they don't represent anything
    #[test]
    pub fn total_value_on_rack() {
        let foo = Rack {
            rack: vec![
                JokersWild,
                RegularTile(Black, Five),
                RegularTile(Black, Six),
                RegularTile(Black, Seven),
            ],
            played_initial_meld: false,
        };
        let expected_score: ScoreValue = ScoreValue::of(30 + 5 + 6 + 7);
        assert_eq!(expected_score, foo.total_value())
    }

    #[test]
    pub fn largest_run_on_rack() {
        let four_to_nine_black = Run::of(Four, Black, 5).expect("Run::Of must succeed");
        let one_to_three_blue = Run::of(One, Blue, 3).expect("Run::Of must succeed");

        let mut tiles = vec![];
        tiles.extend(four_to_nine_black.decompose());
        tiles.extend(one_to_three_blue.decompose());
        tiles.push(RegularTile(Blue, One)); // Duplicate
        tiles.push(RegularTile(Red, Five)); // Island
        tiles.push(RegularTile(Orange, Ten)); // Almost
        tiles.push(RegularTile(Orange, Eleven));
        tiles.push(RegularTile(Orange, Thirteen));
        let mut rng = thread_rng();
        tiles.shuffle(&mut rng);
        let mut test_rack = Rack {
            rack: tiles,
            played_initial_meld: true,
        };

        let run: Run = test_rack
            .get_largest_run()
            .expect("Should find manufactured run");
        assert_eq!(four_to_nine_black, run);
        test_rack = test_rack
            .remove(&four_to_nine_black)
            .expect("Rack was manufactured");
        let second_run = test_rack
            .get_largest_run()
            .expect("Should find manufactured run");
        assert_eq!(one_to_three_blue, second_run);
        test_rack = test_rack
            .remove(&one_to_three_blue)
            .expect("Rack was manufactured");
        assert_eq!(
            ScoreValue::of(1 + 5 + 10 + 11 + 13),
            test_rack.total_value(),
            "Rack should be remaining tiles 1+5+10+11+13"
        );
        assert_eq!(Count(5), test_rack.count().expect("Rack must be countable"))
    }
}
