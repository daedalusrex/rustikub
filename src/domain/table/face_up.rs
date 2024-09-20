use crate::domain::sets::Set;
use crate::domain::tiles::Tile;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

///A layout is a selection of certain sets, representing a particular permutation of their possible configuration
/// main feature is to verify that after manipulating the table, the new layout is a valid version of the old one
/// and or add/determine the difference with a single new tile.
pub struct Layout;

#[derive(Debug, Clone, PartialEq)]
pub struct FaceUpTiles {
    // Publicly viewable and mutable by all players, has all the sets that have been placed
    pub sets: Vec<Set>,
}

use crate::domain::score_value::ScoringRule::OnTable;
use crate::domain::score_value::{ScoreValue, ScoringRule};
use crate::domain::sets::group::Group;
use crate::domain::sets::run::{Run, Slot};
use crate::domain::tiles::tile_sequence::TileSequence;
use crate::domain::{Count, Decompose, RummikubError};
use colored;
use colored::{ColoredString, Colorize};
use ScoringRule::OnRack;

impl Display for FaceUpTiles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for s in &self.sets {
            write!(f, "{}\n", s)?
        }
        Ok(())
    }
}

impl Decompose for FaceUpTiles {
    fn decompose(&self) -> Vec<Tile> {
        let mut tiles: TileSequence = vec![];
        for set in &self.sets {
            tiles.append(set.decompose().as_mut())
        }
        tiles
    }

    fn score(&self, rule: ScoringRule) -> Result<ScoreValue, RummikubError> {
        match rule {
            OnRack => Err(RummikubError),
            OnTable => {
                let sum = self
                    .sets
                    .iter()
                    .map(|s| s.score(OnTable).unwrap().as_u16())
                    .sum::<u16>();

                Ok(ScoreValue::of(sum)?)
            }
        }
    }
}

impl FaceUpTiles {
    pub fn new() -> FaceUpTiles {
        FaceUpTiles { sets: vec![] }
    }

    pub fn valid_rearrangement(
        &self,
        mut with_added_tiles: TileSequence,
        other_face_ups: &FaceUpTiles,
    ) -> bool {
        with_added_tiles.append(&mut self.decompose());
        with_added_tiles.sort();

        let mut other_tiles = other_face_ups.decompose();
        other_tiles.sort();
        with_added_tiles == other_tiles
    }

    pub fn runs(&self) -> Vec<&Run> {
        self.sets
            .iter()
            .filter_map(|x| match x {
                Set::Group(_) => None,
                Set::Run(r) => Some(r),
            })
            .collect()
    }

    pub fn groups(&self) -> Vec<Group> {
        self.sets
            .iter()
            .filter_map(|x| match x {
                Set::Group(g) => Some(g),
                Set::Run(_) => None,
            })
            .cloned()
            .collect()
    }

    /// What a type signature wow
    pub fn all_possible_slots(&self) -> Option<HashMap<Tile, Vec<(Slot, &Set)>>> {
        todo!()
    }

    /// What a type signature wow
    pub fn all_spares(&self) -> Option<HashMap<Tile, Vec<&Set>>> {
        todo!()
    }

    #[deprecated]
    pub fn simple_add_tile(&self, candidate: &Tile) -> Option<FaceUpTiles> {
        let mut mut_sets: Vec<Set> = vec![];
        let mut tile_was_added = false;

        // TODO add as debugging: // println!("Attempting to place {}", candidate);

        for existing_set in &self.sets {
            if tile_was_added {
                mut_sets.push(existing_set.clone());
            } else {
                let set_with_added: Option<Set> = match existing_set {
                    Set::Group(g) => {
                        if let Some(updated_group) = g.insert_tile(candidate) {
                            tile_was_added = true;
                            Some(Set::Group(updated_group))
                        } else {
                            None
                        }
                    }
                    Set::Run(r) => {
                        if let Some(updated_run) = r.add_tile(candidate, None) {
                            tile_was_added = true;
                            Some(Set::Run(updated_run))
                        } else {
                            None
                        }
                    }
                };
                if let Some(set_with_added) = set_with_added {
                    mut_sets.push(set_with_added.clone())
                } else {
                    mut_sets.push(existing_set.clone())
                }
            }
        }

        if tile_was_added {
            Some(FaceUpTiles { sets: mut_sets })
        } else {
            None
        }
    }

    pub fn place_new_sets(&self, sets: &Vec<Set>) -> FaceUpTiles {
        let mut new_face_up = self.clone();
        for set in sets {
            new_face_up.place_set(set.clone())
        }
        new_face_up
    }

    /// Privately modifies self to add a new set
    fn place_set(&mut self, set: Set) {
        self.sets.push(set);
    }
}

#[cfg(test)]
mod basic_face_up_tests {
    use super::*;
    use crate::domain::sets::group::Group;
    use crate::domain::sets::run::Run;
    use crate::domain::tiles::color::Color::*;
    use crate::domain::tiles::number::Number::*;
    use crate::domain::tiles::Tile::RegularTile;

    #[test]
    pub fn decompose_works() {
        let run = Run::of(One, Blue, 3).expect("BROKEN");
        let group = Group::of(Two, &vec![Blue, Black, Orange]).expect("BROKEN");

        let mut face_up = FaceUpTiles::new();
        face_up.sets.push(Set::Group(group));
        face_up.sets.push(Set::Run(run));

        let mut actual = face_up.decompose();
        actual.sort();

        let mut expected: TileSequence = vec![
            RegularTile(Blue, One),
            RegularTile(Blue, Two),
            RegularTile(Blue, Three),
            RegularTile(Blue, Two),
            RegularTile(Black, Two),
            RegularTile(Orange, Two),
        ];
        expected.sort();

        assert_eq!(actual.len(), 6);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_valid_rearrangement() {
        let run = Run::of(One, Blue, 3).unwrap();
        let group = Group::of(Four, &vec![Blue, Black, Orange, Red]).unwrap();

        let original = FaceUpTiles {
            sets: vec![Set::Run(run.clone()), Set::Group(group.clone())],
        };

        let expected_run = Run::of(One, Blue, 4).expect("BROKEN");
        let expected_group = Group::of(Four, &vec![Black, Orange, Red]).unwrap();
        let expected_face_up = FaceUpTiles {
            sets: vec![
                Set::Run(expected_run.clone()),
                Set::Group(expected_group.clone()),
            ],
        };
        assert!(original.valid_rearrangement(vec![], &expected_face_up));
    }

    #[test]
    fn test_runs_groups_filter() {
        let run = Run::of(One, Blue, 3).unwrap();
        let group = Group::of(Four, &vec![Blue, Black, Orange, Red]).unwrap();

        let original = FaceUpTiles {
            sets: vec![Set::Run(run.clone()), Set::Group(group.clone())],
        };
        assert_eq!(vec![&run], original.runs());
        assert_eq!(vec![group], original.groups());
    }
}
