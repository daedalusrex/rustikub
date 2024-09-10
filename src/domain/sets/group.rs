use crate::domain::score_value::ScoringRule::OnRack;
use crate::domain::score_value::{ScoreValue, ScoringRule};
use crate::domain::tiles::color::Color;
use crate::domain::tiles::number::Number;
use crate::domain::tiles::Tile;
use crate::domain::tiles::Tile::RegularTile;
use crate::domain::{Count, Decompose, RummikubError};
use std::collections::HashSet;
use std::fmt::Display;
use ScoringRule::OnTable;
use Tile::JokersWild;

const MAX_GROUP_SIZE: usize = 4;
const MIN_GROUP_SIZE: usize = 3;
const MAX_JOKERS_IN_GROUP: u8 = 2;

///A set of either three or four tiles of the same number in different colors.
#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    num: Number,
    colors: HashSet<Color>,
    jokers: u8,
}

impl Group {
    /// Creates a group based on defining parameters as given
    /// Interprets duplicate colors as only one of that color
    pub fn of(num: Number, colors: &Vec<Color>) -> Option<Group> {
        if colors.len() > MAX_GROUP_SIZE {
            return None;
        }
        let mut cols_set = HashSet::new();
        for &col in colors {
            cols_set.insert(col);
        }
        if cols_set.len() < MIN_GROUP_SIZE {
            return None;
        }
        Some(Group {
            num,
            jokers: 0,
            colors: cols_set,
        })
    }

    /// Checks the given candidate tiles against a logical constraints that define a Group
    /// If successful returns a Group composed of those tiles, otherwise None
    /// TODO candidates argument should be a reference
    pub fn parse(candidates: Vec<Tile>) -> Option<Group> {
        if candidates.len() > MAX_GROUP_SIZE || candidates.len() < MIN_GROUP_SIZE {
            return None;
        }

        let mut group_number: Number = Number::One;
        let mut num_jokers: u8 = 0;
        let mut cols = HashSet::new();
        let first_num: Number;

        let first_no_joke = candidates.iter().filter(|tile| !tile.is_joker()).next()?;
        match first_no_joke {
            RegularTile(c, n) => first_num = n.clone(),
            JokersWild => return None,
        }

        // Find the first regular tile, that has a number
        for tile in candidates {
            match tile {
                JokersWild => num_jokers += 1,
                RegularTile(color, num) => {
                    if first_num != num {
                        return None;
                    }
                    group_number = num;
                    if cols.contains(&color) {
                        return None;
                    }
                    cols.insert(color);
                }
            }
        }
        if num_jokers > MAX_JOKERS_IN_GROUP {
            return None;
        }
        Some(Group {
            num: group_number,
            colors: cols,
            jokers: num_jokers,
        })
    }

    pub fn contains(&self, c: Color) -> bool {
        // TODO Jokers?
        self.colors.contains(&c)
    }

    pub fn get_group_num(&self) -> Number {
        self.num
    }

    pub fn add_tile(&self, tile: &Tile) -> Option<Group> {
        if self.colors.len() + self.jokers as usize == MAX_GROUP_SIZE {
            return None;
        }
        match tile {
            JokersWild => {
                if self.jokers == MAX_JOKERS_IN_GROUP {
                    return None;
                }
                let jokers = self.jokers + 1;
                Some(Group {
                    num: self.num,
                    colors: self.colors.clone(),
                    jokers,
                })
            }
            RegularTile(color, num, ..) => {
                if self.contains(*color) || &self.num != num {
                    return None;
                }
                let mut colors = self.colors.clone();
                colors.insert(color.clone());
                Some(Group {
                    num: self.num,
                    colors,
                    jokers: self.jokers,
                })
            }
        }
    }

    pub fn has_spare(&self) -> bool {
        self.count().unwrap().0 as usize > MIN_GROUP_SIZE
    }

    pub fn extract_spare(&self, color: Color) -> Option<(Group, Tile)> {
        // If all regular tiles, then it will have all colors, but if one is joker
        // then it would not be allowed to return the joker, as they must be "retrieved"
        if !self.has_spare() || !self.contains(color) {
            return None;
        }
        let mut new_group = self.clone();
        new_group.colors.remove(&color);
        Some((new_group, RegularTile(color, self.num)))
    }

    /// The only way to "retrieve" the joker is to replace the color it represents
    /// with a regular tile that forms a valid Group. Either Color in a Group of 3 is acceptable
    /// If successful returns the new run and a Joker Tile
    pub fn retrieve_joker(&self, tile: Tile) -> Option<(Group, Tile)> {
        todo!()
    }
}

impl Decompose for Group {
    fn decompose(&self) -> Vec<Tile> {
        let mut composite_tiles: Vec<Tile> = vec![];
        for joker in 0..self.jokers {
            composite_tiles.push(JokersWild);
        }
        for color in &self.colors {
            composite_tiles.push(RegularTile(*color, self.num))
        }
        composite_tiles
    }

    fn count(&self) -> Result<Count, RummikubError> {
        Ok(Count(self.colors.len() as u8 + self.jokers))
    }
    fn score(&self, rule: ScoringRule) -> Result<ScoreValue, RummikubError> {
        match rule {
            OnRack => self.decompose().score(OnRack),
            OnTable => Ok(self.num.as_value() * self.count()?.0 as u16),
        }
    }
}

#[cfg(test)]
pub mod group_tests {
    use super::*;
    use crate::domain::tiles::color::Color;
    use crate::domain::tiles::number::Number;
    use crate::domain::tiles::Tile::{JokersWild, RegularTile};
    use std::vec;
    use Color::*;
    use Number::*;

    fn object_mother_good_group_of_three() -> Vec<Tile> {
        let color = Color::get_rand();
        let num = Number::get_rand();
        let first = RegularTile(color, num);
        let second = RegularTile(color.next(), num);
        let third = RegularTile(color.next().next(), num);
        vec![first, second, third]
    }

    #[test]
    fn test_parsing_good() {
        let success = object_mother_good_group_of_three();
        assert_ne!(None, Group::parse(success.clone()));
        if let Some(good_group) = Group::parse(success.clone()) {
            assert_eq!(success.len() as u8, good_group.count().unwrap().0);
            if let Some(RegularTile(_, num)) = success.first() {
                assert_eq!(num, &good_group.num)
            } else {
                panic!("Test is Broken!!!! There should always be something there")
            }

            //Colors in candidates match colors in Group
            for tile in &success {
                if let RegularTile(color, _) = tile {
                    assert!(good_group.contains(*color));
                } else {
                    panic!("Test Broken! Should be No Jokers Here!")
                }
            }
        }

        let mut with_joker = success.clone();
        with_joker.insert(0, JokersWild);
        let joker_group = Group::parse(with_joker.clone());
        assert_ne!(None, joker_group);
    }

    #[test]
    fn test_parsing_reject_bad() {
        // Size Constraints
        let normal = object_mother_good_group_of_three();
        let mut too_big = normal.clone();
        too_big.append(&mut vec![Tile::any_regular(), Tile::any_regular()]);
        let mut too_small = normal.clone();
        too_small.pop();
        assert_eq!(None, Group::parse(too_big));
        assert_eq!(None, Group::parse(too_small));

        // Different Numbers, Allowable Colors
        let bad_nums = vec![
            RegularTile(Red, One),
            RegularTile(Blue, Two),
            RegularTile(Black, Three),
        ];

        assert_eq!(None, Group::parse(bad_nums));

        // Same Numbers, Duplicate Colors
        let duped_colors = vec![
            RegularTile(Red, One),
            RegularTile(Red, One),
            RegularTile(Black, One),
        ];

        assert_eq!(None, Group::parse(duped_colors));
    }

    #[test]
    fn test_jokers_parsing() {
        let mut base = object_mother_good_group_of_three();
        // Good Group with one additional joker should succeed!
        base.push(JokersWild);
        assert_ne!(None, Group::parse(base.clone()));
        // Group of 5 with Two Jokers should fail
        base.push(JokersWild);
        assert_eq!(None, Group::parse(base.clone()));
        // Two Jokers is legitimate!
        assert_ne!(
            None,
            Group::parse(vec![Tile::any_regular(), JokersWild, JokersWild])
        );
        // Three Jokers is not
        assert_eq!(None, Group::parse(vec![JokersWild, JokersWild, JokersWild]));
        // One Joker Should Succeed
        let mut just_one_joker = object_mother_good_group_of_three();
        just_one_joker.pop();
        just_one_joker.push(JokersWild);
        assert_ne!(None, Group::parse(just_one_joker));
    }

    #[test]
    fn composites_match() {
        let mut origin = object_mother_good_group_of_three();
        origin.sort();
        let my_group = Group::parse(origin.clone()).unwrap();
        let mut output = my_group.decompose();
        output.sort();
        assert_eq!(origin, output);

        let mut with_joker = origin.clone();
        with_joker.push(JokersWild);
        with_joker.sort();
        let joker_group = Group::parse(with_joker.clone()).unwrap();
        let mut joker_out = joker_group.decompose();
        joker_out.sort();
        assert_eq!(with_joker, joker_out);
    }

    #[test]
    fn scoring_vals() {
        let known_group = Group::parse(vec![
            RegularTile(Red, Five),
            RegularTile(Blue, Five),
            JokersWild,
        ])
        .unwrap();
        assert_eq!(
            ScoreValue::of_u16(15u16),
            known_group.score(OnTable).unwrap()
        );
        assert_eq!(
            ScoreValue::of_u16(40u16),
            known_group.score(OnRack).unwrap()
        );
    }

    #[test]
    fn add_tile_happy() {
        let known_group = Group::parse(vec![
            RegularTile(Red, Five),
            RegularTile(Blue, Five),
            RegularTile(Orange, Five),
        ])
        .unwrap();
        let result = known_group.add_tile(&RegularTile(Black, Five));
        assert!(result.is_some());

        let parsed = Group::parse(vec![
            RegularTile(Red, Five),
            RegularTile(Blue, Five),
            RegularTile(Orange, Five),
            RegularTile(Black, Five),
        ])
        .unwrap();
        assert_eq!(parsed, result.unwrap());

        let joker_g = Group::parse(vec![
            RegularTile(Red, Five),
            RegularTile(Blue, Five),
            JokersWild,
        ])
        .unwrap();
        let joke = joker_g.add_tile(&RegularTile(Orange, Five));
        assert!(joke.is_some());
        let joke_jok = joker_g.add_tile(&JokersWild);
        assert!(joke_jok.is_some());
    }

    #[test]
    fn test_spares() {
        let gang_of_four = Group::of(Four, &vec![Red, Orange, Black, Blue]).unwrap();

        assert!(gang_of_four.has_spare());
        let expected = Some((
            Group::of(Four, &vec![Red, Black, Orange]).unwrap(),
            RegularTile(Blue, Four),
        ));
        assert_eq!(gang_of_four.extract_spare(Blue), expected);

        let gang_of_four_joker = Group::parse(vec![
            RegularTile(Red, Four),
            RegularTile(Orange, Four),
            JokersWild,
            RegularTile(Blue, Four),
        ])
        .unwrap();
        assert!(gang_of_four_joker.has_spare());
        assert!(gang_of_four_joker.extract_spare(Black).is_none());
        let expected = Some((
            Group::parse(vec![
                RegularTile(Red, Four),
                RegularTile(Orange, Four),
                JokersWild,
            ])
            .unwrap(),
            RegularTile(Blue, Four),
        ));
        assert_eq!(gang_of_four_joker.extract_spare(Blue), expected);
    }
}
