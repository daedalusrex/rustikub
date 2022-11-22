use crate::domain::sets::Set;

///A layout is a selection of certain sets, representing a particular permutation of their possible configuration
/// main feature is to verify that after manipulating the table, the new layout is a valid version of the old one
/// and or add/determine the difference with a single new tile.
pub struct Layout;

#[derive(Debug, Clone)]
pub struct FaceUpTiles {
    // Publicly viewable and mutable by all players, has all the sets that have been placed
    pub sets: Vec<Set>,
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

// TODO Implement Display