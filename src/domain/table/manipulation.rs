use crate::domain::player::rack::Rack;
use crate::domain::sets::group::Group;
use crate::domain::sets::run::Run;
use crate::domain::sets::Set;
use crate::domain::table::face_up::FaceUpTiles;
use crate::domain::tiles::tile_sequence::{TileSequence, TileSequenceType};
use crate::domain::tiles::Tile;
use crate::domain::{Count, Decompose};
use std::os::unix::raw::time_t;

/// If possible, places one (or more) tiles from the rack into the face up tiles on the table
/// Returns the new Rack and New Tiles if successful, otherwise returns None,
/// indicating no change could be made
/// TODO this is the one actually called, and would be a great place to "instantiate" or provide
/// a strategy
pub fn rearrange(rack: &Rack, table: &FaceUpTiles) -> Option<(Rack, FaceUpTiles)> {
    let chosen_fn: fn(&Rack, &FaceUpTiles) -> Option<(Rack, FaceUpTiles)> = human_like_algorithm;

    chosen_fn(rack, table)
}

fn human_like_algorithm(rack: &Rack, table: &FaceUpTiles) -> Option<(Rack, FaceUpTiles)> {
    let mut remaining = TileSequenceType::of(&rack.decompose());
    let mut added: TileSequence = vec![];
    let mut groups: Vec<Group> = vec![];
    let mut runs: Vec<Run> = vec![];

    // todo as iter? -> probably ugly
    // This too many layers, many logic bugs need redo kinda  lot, break it up and so on
    // Honestly need to redo Face Up to make modifications and other access actually easier
    //  -- Modification of a single run etc. Like, with a fucking reference
    // Hold on there cowboy. making opaque modifications to a referenced run in the table
    // is an easy way to create invalid states that needs to be checked. Indexing and then resending
    // update request is also unpleasant though
    'outer_runs: for run in table.runs() {
        let count = added.len();
        'inner_rack_tiles: for tile in remaining.decompose() {
            if let Some(edges) = run.edge_slots() {
                if edges.contains_key(&tile) {
                    let new_run = run
                        .insert_tile_on_edge(tile, edges[&tile])
                        .expect("Insert Edge Failure");
                    runs.push(new_run);
                    remaining = remaining.remove(&tile).expect("Removal Failure");
                    added.push(tile);
                    /* Original had a bug If you have duplicate tiles (e.g. 2 Red,One in rack)
                    and a run 2->7Red, you will add the first tile to the original run (outer for loop)
                    add it to the new face up
                    Then add the second tile to the same one in the outer for loop.
                    */
                    // The run was modified, and therefore we'd need to compare to it again
                    break 'inner_rack_tiles;
                }
            }
        }
        if added.len() == count {
            runs.push(run.clone());
        }
    }

    'outer_groups: for group in table.groups() {
        let count = added.len();
        'inner_rack_tiles: for tile in remaining.decompose() {
            if let Some(new_group) = group.insert_tile(&tile) {
                groups.push(new_group);
                remaining = remaining.remove(&tile).expect("Removal Failure");
                added.push(tile);
                break 'inner_rack_tiles; // Same reasoning as runs above
            }
        }
        if added.len() == count {
            groups.push(group);
        }
    }

    let mut new_arrangement: Vec<Set> = vec![];
    new_arrangement.extend(runs.into_iter().map(|r| Set::Run(r)));
    new_arrangement.extend(groups.into_iter().map(|g| Set::Group(g)));
    let new_table = FaceUpTiles {
        sets: new_arrangement,
    };

    assert!(table.valid_rearrangement(added, &new_table));

    if remaining.count().ok()? < rack.count().ok()? {
        return Some((
            Rack::new(&remaining.0, Some(rack.played_initial_meld)).ok()?,
            new_table,
        ));
    }
    None
}

fn place_new_tiles_simple(rack: &Rack, table: &FaceUpTiles) -> Option<(Rack, FaceUpTiles)> {
    // Place New Tiles simple
    let mut mut_face_up = table.clone();
    let mut remaining = TileSequenceType::of(&rack.decompose());

    for attempt in remaining.decompose() {
        if let Some(place_success) = mut_face_up.simple_add_tile(&attempt) {
            mut_face_up = place_success;
            remaining = remaining.remove(&attempt)?;
        }
    }
    if remaining.count().ok()? < rack.count().ok()? {
        return Some((
            Rack::new(&remaining.0, Some(rack.played_initial_meld)).ok()?,
            mut_face_up,
        ));
    }
    None
}

/// Simples possible version of the algorithm. If I have something decomposable (presumably the
/// face up tiles), shatter it into it's individual components, and then attempt to
/// put it back together again, but including the new tile.
/// Simple implementation creates all runs from largest to smallest, and then groups
fn shatter_and_recombobulate(
    candidates: &impl Decompose,
    initial_table: &impl Decompose,
) -> Option<(TileSequenceType, FaceUpTiles)> {
    let mut remaining_tiles = TileSequenceType::of(&initial_table.decompose());
    remaining_tiles.0.append(&mut candidates.decompose());
    let mut sets: Vec<Set> = vec![];

    let mut optional_run = remaining_tiles.largest_run();
    while let Some(ref largest_run) = optional_run {
        sets.push(Set::Run(largest_run.clone()));
        remaining_tiles = remaining_tiles
            .remove(largest_run)
            .expect("Must be able to remove the found run");
        optional_run = remaining_tiles.largest_run();
    }

    let mut optional_group = remaining_tiles.largest_group();
    while let Some(ref largest_group) = optional_group {
        sets.push(Set::Group(largest_group.clone()));
        remaining_tiles = remaining_tiles
            .remove(largest_group)
            .expect("Must be able to remove the found group");
        optional_group = remaining_tiles.largest_group();
    }

    let possible_table = FaceUpTiles { sets };

    // If you did not add any new tiles to the table, it's not valid i.e. unchanged
    if possible_table.count().ok()? <= initial_table.count().ok()? {
        return None;
    }

    let original_candidates = candidates.decompose();
    for remaining_tile in remaining_tiles.0.iter() {
        // It's illegal to remove tiles from the board (swapping)
        // Therefore, enforce that every tile which is remaining started
        // out in the original candidates (i.e. rack)
        if !original_candidates.contains(remaining_tile) {
            return None;
        }
    }
    Some((remaining_tiles, possible_table))
}

#[cfg(test)]
mod example_manipulation_tests_from_rulebook {
    use super::*;
    use crate::domain;
    use crate::domain::player::rack::Rack;
    use crate::domain::score_value::ScoreValue;
    use crate::domain::sets::group::Group;
    use crate::domain::sets::run::Run;
    use crate::domain::sets::Set::*;
    use crate::domain::table::face_up::FaceUpTiles;
    use crate::domain::tiles::color::Color::*;
    use crate::domain::tiles::number::Number::*;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use crate::domain::tiles::*;
    use crate::domain::Decompose;

    // TODO These should be parameterized tests, as the only difference is the input and output
    // But they do have complex constructors, sooo

    /// Blue 4,5,6 are on the table. The player  adds a blue 3. The blue 8 is added to the
    /// group of 8’s already on the table.
    #[test]
    pub fn add_tile_to_make_new_set() {
        let example_rack = Rack::new(
            &vec![RegularTile(Blue, Three), RegularTile(Blue, Eight)],
            None,
        )
        .expect("TEST");
        let example_table: FaceUpTiles = FaceUpTiles {
            sets: vec![
                Run(Run::of(Four, Blue, 3).expect("TEST")),
                Group(Group::of(Eight, &vec![Red, Orange, Black]).expect("TEST")),
            ],
        };

        let actual = rearrange(&example_rack, &example_table);

        assert!(actual.is_some());
        // TODO fix the fact that order matters
        let expected_table = FaceUpTiles {
            sets: vec![
                Run(Run::of(Three, Blue, 4).expect("TEST")),
                Group(Group::of(Eight, &vec![Red, Orange, Black, Blue]).expect("TEST")),
            ],
        };
        let (actual_rack, actual_table) = actual.unwrap();
        assert!(actual_rack.is_empty());
        assert_eq!(expected_table, actual_table);
    }

    /// A tile is missing from the potential blue run on the rack. The player takes the blue 4
    /// from the group of four on the table and lays the run: blue 3,4,5,6.
    #[test]
    pub fn remove_and_use_fourth_to_create_new_set() {
        let test_rack = Rack::new(
            &vec![
                RegularTile(Blue, Three),
                RegularTile(Blue, Five),
                RegularTile(Blue, Six),
            ],
            None,
        )
        .expect("TEST");

        let test_table = FaceUpTiles {
            sets: vec![Group(
                Group::of(Four, &vec![Red, Orange, Black, Blue]).expect("TEST"),
            )],
        };

        let expected_table = FaceUpTiles {
            sets: vec![Group(
                Group::of(Four, &vec![Red, Orange, Black, Blue]).expect("TEST"),
            )],
        };
        let actual = rearrange(&test_rack, &test_table);

        assert!(actual.is_some());
        let (actual_rack, actual_table) = actual.expect("TEST");
        assert!(actual_rack.is_empty());
        assert_eq!(expected_table, actual_table)
    }

    /// The player adds a blue 11 to the run and uses the 8’s to form a new group
    #[test]
    pub fn add_fourth_and_remove_tile_to_create_new_set() {
        let test_rack = Rack::new(
            &vec![
                RegularTile(Blue, Eleven),
                RegularTile(Black, Eight),
                RegularTile(Orange, Eight),
            ],
            None,
        )
        .expect("TEST");

        let test_table = FaceUpTiles {
            sets: vec![Run(Run::of(Eight, Blue, 3).expect("TEST"))],
        };

        let actual = rearrange(&test_rack, &test_table);

        assert!(actual.is_some());
        let (actual_rack, actual_table) = actual.expect("TEST");
        let expected_table = FaceUpTiles {
            sets: vec![
                Run(Run::of(Nine, Blue, 4).expect("TEST")),
                Group(Group::of(Eight, &vec![Orange, Black, Blue]).expect("TEST")),
            ],
        };
        assert!(actual_rack.is_empty());
        assert_eq!(expected_table, actual_table)
    }

    /// Splitting a run, The player splits the run and uses the red 6 to form two new runs.
    #[test]
    pub fn splitting_a_run() {
        let test_rack = Rack::new(&vec![RegularTile(Red, Six)], None).expect("TEST");

        let test_table = FaceUpTiles {
            sets: vec![Run(Run::of(Four, Red, 5).expect("TEST"))],
        };

        let actual = rearrange(&test_rack, &test_table);

        assert!(actual.is_some());
        let (actual_rack, actual_table) = actual.expect("TEST");
        let expected_table = FaceUpTiles {
            sets: vec![
                Run(Run::of(Four, Red, 3).expect("TEST")),
                Run(Run::of(Six, Red, 3).expect("TEST")),
            ],
        };
        assert!(actual_rack.is_empty());
        assert_eq!(expected_table, actual_table)
    }

    /// The player places a blue 1 from the rack with the orange 1 from the run and the red 1 from
    /// the group to form a new group.
    #[test]
    pub fn combined_split() {
        let test_rack = Rack::new(&vec![RegularTile(Blue, One)], None).expect("TEST");

        let test_table = FaceUpTiles {
            sets: vec![
                Run(Run::of(One, Orange, 4).expect("TEST")),
                Group(Group::of(One, &vec![Blue, Black, Red, Orange]).expect("TEST")),
            ],
        };

        let actual = rearrange(&test_rack, &test_table);

        assert!(actual.is_some());
        let (actual_rack, actual_table) = actual.expect("TEST");
        let expected_table = FaceUpTiles {
            sets: vec![
                Run(Run::of(Two, Orange, 3).expect("TEST")),
                Group(Group::of(One, &vec![Black, Blue, Orange]).expect("TEST")),
                Group(Group::of(One, &vec![Blue, Red, Orange]).expect("TEST")),
            ],
        };
        assert!(actual_rack.is_empty());
        assert_eq!(expected_table, actual_table)
    }

    /// The player manipulates the three existing sets on the table, and use the black 10 and
    /// the blue 5 from the rack to make three groups and one new run.
    #[test]
    pub fn multiple_split() {
        let test_rack = Rack::new(
            &vec![RegularTile(Black, Ten), RegularTile(Blue, Five)],
            None,
        )
        .expect("TEST");

        let test_table = FaceUpTiles {
            sets: vec![
                Run(Run::of(Five, Orange, 3).expect("TEST")),
                Run(Run::of(Five, Red, 3).expect("TEST")),
                Run(Run::of(Five, Black, 5).expect("TEST")),
            ],
        };

        let actual = rearrange(&test_rack, &test_table);

        assert!(actual.is_some());
        let (actual_rack, actual_table) = actual.expect("TEST");
        let expected_table = FaceUpTiles {
            sets: vec![
                Group(Group::of(Five, &vec![Blue, Orange, Red, Black]).expect("TEST")),
                Group(Group::of(Six, &vec![Orange, Red, Black]).expect("TEST")),
                Group(Group::of(Seven, &vec![Orange, Red, Black]).expect("TEST")),
                Run(Run::of(Five, Black, 5).expect("TEST")),
            ],
        };
        assert!(actual_rack.is_empty());
        assert_eq!(expected_table, actual_table)
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
    #[ignore]
    pub fn can_replace_joker_with_black_or_yellow_three() {
        todo!()
    }

    /// The player splits the run and clears the joker. For the record, we've never played it this way
    /// and it blows my mind
    #[test]
    #[ignore]
    pub fn can_split_run_and_implicitly_extract_joker() {
        todo!()
    }

    ///The player adds the blue 5 and clears the joker.
    #[test]
    #[ignore]
    pub fn can_simply_replace_joker_with_blue_five() {
        todo!()
    }

    ///The player splits the run. He moves the black 1 to the group of ones, he moves the
    /// black 2 to the group of twos and frees the joker
    #[test]
    #[ignore]
    pub fn can_manipulate_table_without_tile_from_rack_and_implicitly_free_joker() {
        todo!()
    }
}
