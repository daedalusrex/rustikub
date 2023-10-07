use std::fmt::{Display, Formatter};
use crate::domain::sets::Set;
use crate::domain::tiles::Tile;

///A layout is a selection of certain sets, representing a particular permutation of their possible configuration
/// main feature is to verify that after manipulating the table, the new layout is a valid version of the old one
/// and or add/determine the difference with a single new tile.
pub struct Layout;

#[derive(Debug, Clone)]
pub struct FaceUpTiles {
    // Publicly viewable and mutable by all players, has all the sets that have been placed
    pub sets: Vec<Set>,
}

use colored;
use colored::{ColoredString, Colorize};

impl Display for FaceUpTiles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for s in &self.sets {
            write!(f, "{}\n", s).unwrap()
        }
        Ok(())
    }
}

impl FaceUpTiles {
    /// Rules have several types of manipulations
    /// Add one or more tiles from rack to make new set
    /// Remove a fourth tile from a group and use it to form a new set:
    /// Add a fourth tile to a set and remove one tile from it, to make another set:
    /// Splitting a run
    /// Combined split
    /// Multiple split:
    /// Also, special joker rules need to be considered
    pub fn manipulate() {
        todo!()
    }

    pub fn new() -> FaceUpTiles {
        FaceUpTiles { sets: vec![] }
    }

    /// Given a candidate tile check if any of the above listed manipulations
    /// can result in a layout that has the tile as part of all the face up sets
    /// TODO for now, just do the simplest possible steps of adding to existing sets
    pub fn place_new_tile(&self, candidate: &Tile) -> Option<FaceUpTiles> {
        let mut mut_sets = vec![];
        let mut change_occured = false;

        for existing_set in &self.sets {
            let add_attempt = match existing_set {
                Set::Group(g) => {
                    if let Some(updated_group) = g.add_tile(candidate) {
                        Set::Group(updated_group)
                    } else {
                        existing_set.clone()
                    }
                }
                Set::Run(r) => {
                    if let Some(updated_run) = r.add_tile(candidate, None) {
                        Set::Run(updated_run)
                    } else {
                        existing_set.clone()
                    }
                }
            };
            if &add_attempt != existing_set {
                change_occured = true;
                // TODO just adding to the first possibility may not be the best strategy
                break;
            }
            mut_sets.push(add_attempt);
        }

        if change_occured {
            return Some(FaceUpTiles{ sets: mut_sets });
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
