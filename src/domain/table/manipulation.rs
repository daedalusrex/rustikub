use crate::domain::player::rack::Rack;
use crate::domain::table::face_up::FaceUpTiles;

pub fn rearrange(rack: Rack, table: FaceUpTiles) -> Option<(Rack, FaceUpTiles)> {
    // TODO add some kind of grand decomposition, and then recompose the table set by set
    // should be quite similar to what rack does, but on a grander scale. (ignoring the joker)
    None
}

#[cfg(test)]
mod example_manipulation_tests_from_rulebook {
    use super::*;
    use crate::domain;
    use crate::domain::player::rack::Rack;
    use crate::domain::score_value::ScoreValue;
    use crate::domain::sets::group::Group;
    use crate::domain::sets::run::Run;
    use crate::domain::sets::Set;
    use crate::domain::table::face_up::FaceUpTiles;
    use crate::domain::tiles::color::Color::*;
    use crate::domain::tiles::number::Number::*;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use crate::domain::tiles::*;
    use crate::domain::Decompose;

    /// Blue 4,5,6 are on the table. The player  adds a blue 3. The blue 8 is added to the
    /// group of 8’s already on the table.
    #[test]
    pub fn add_tile_to_make_new_set() {
        let example_rack: Rack = Rack {
            rack: vec![RegularTile(Blue, Three), RegularTile(Blue, Eight)],
            played_initial_meld: true,
        };
        let example_table: FaceUpTiles = FaceUpTiles {
            sets: vec![Set::Group(
                Group::of(&Eight, &vec![Red, Orange, Black]).unwrap(),
            )],
        };

        let op_result = rearrange(example_rack, example_table);
        assert!(op_result.is_some());
        let (actual_rack, actual_table) = op_result.unwrap();
        assert!(actual_rack.is_empty());
        // TODO make assertions on state of table as well
    }

    /// A tile is missing from the potential blue run on the rack. The player takes the blue 4
    /// from the group of four on the table and lays the run: blue 3,4,5,6.
    #[test]
    pub fn remove_and_use_fourth_to_create_new_set() {
        panic!()
    }

    /// The player adds a blue 11 to the run and uses the 8’s to form a new group
    #[test]
    pub fn add_fourth_and_remove_tile_to_create_new_set() {
        panic!()
    }

    /// Splitting a run, The player splits the run and uses the red 6 to form two new runs.
    #[test]
    pub fn splitting_a_run() {
        panic!()
    }

    /// The player places a blue 1 from the rack with the orange 1 from the run and the red 1 from
    /// the group to form a new group.
    #[test]
    pub fn combined_split() {
        panic!()
    }

    /// The player manipulates the three existing sets on the table, and use the black 10 and
    /// the blue 5 from the rack to make three groups and one new run.
    #[test]
    pub fn multiple_split() {
        panic!()
    }
}

#[cfg(test)]
mod example_clearing_joker_from_rulebook {
    use super::*;
    use crate::domain::score_value::ScoreValue;
    use crate::domain::sets::*;
    use crate::domain::tiles::*;
    use crate::domain::Decompose;

    ///The player can replace the joker by each one of the tiles on his rack or by both
    #[test]
    pub fn can_replace_joker_with_black_or_yellow_three() {
        panic!()
    }

    /// The player splits the run and clears the joker. For the record, we've never played it this way
    /// and it blows my mind
    #[test]
    pub fn can_split_run_and_implicitly_extract_joker() {
        panic!()
    }

    ///The player adds the blue 5 and clears the joker.
    #[test]
    pub fn can_simply_replace_joker_with_blue_five() {
        panic!()
    }

    ///The player splits the run. He moves the black 1 to the group of ones, he moves the
    /// black 2 to the group of twos and frees the joker
    #[test]
    pub fn can_manipulate_table_without_tile_from_rack_and_implicitly_free_joker() {
        panic!()
    }
}
