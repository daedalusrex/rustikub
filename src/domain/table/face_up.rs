use crate::domain::sets::Set;
use crate::domain::tiles::Tile;
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

use crate::domain::tiles::tile_sequence::TileSequence;
use crate::domain::{Count, Decompose, RummikubError};
use colored;
use colored::{ColoredString, Colorize};

impl Display for FaceUpTiles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for s in &self.sets {
            let foo = format!("EXPLODED While wirting: {:?}", s);
            write!(f, "{}\n", s).expect(&foo)
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
    /// TODO plan is to eventually add one function for each possible change. The hardest being split
    pub fn place_new_tile(&self, candidate: &Tile) -> Option<FaceUpTiles> {
        let mut mut_sets: Vec<Set> = vec![];
        let mut tile_was_added = false;

        // TODO add as debugging: // println!("Attempting to place {}", candidate);

        for existing_set in &self.sets {
            if tile_was_added {
                mut_sets.push(existing_set.clone());
            } else {
                let set_with_added: Option<Set> = match existing_set {
                    Set::Group(g) => {
                        if let Some(updated_group) = g.add_tile(candidate) {
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
            return Some(FaceUpTiles { sets: mut_sets });
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
}
