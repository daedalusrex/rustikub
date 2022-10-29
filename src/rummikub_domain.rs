#[allow(dead_code)]
pub mod types {

    //106 tiles in the game, including 104 numbered tiles (valued 1 to 13 in four different colors, two copies of each) and two jokers

    use crate::rummikub_domain::types::Tile::{JokersWild, RegularTile};
    use std::collections::HashSet;
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;
    use strum_macros::EnumString;

    #[derive(EnumIter, Debug, PartialEq, Eq, Hash, Copy, Clone)]
    enum Color {
        Red,
        Blue,
        Orange,
        Black,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, EnumIter, Hash, Copy, Clone)]
    enum Number {
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Eleven,
        Twelve,
        Thirteen,
        //TODO, it's tempting to put u8's here, but for now, I'm not going to because
        // I want to make illegal states unrepresentable.
    }

    #[derive(Debug, Hash, Clone)]
    enum Tile {
        RegularTile(Color, Number),
        JokersWild,
    }

    impl Tile {
        fn is_color(&self, color: Color) -> bool {
            if let RegularTile(c, _) = self {
                *c == color
            } else {
                false
            }
        }
        fn is_number(&self, num: Number) -> bool {
            if let Tile::RegularTile(_, n) = self {
                *n == num
            } else {
                false
            }
        }
    }

    enum Set {
        // There are two kinds of sets, either a group or a run
        Group,
        Run,
    }

    struct Group {
        //A set of either three or four tiles of the same number in different colors.
        members: HashSet<Tile>,
    }
    impl Group {
        fn new(candidates: Vec<Tile>) -> Option<Group> {
            // TODO something like parsing with the new operator here to enforce characteristics
            let mut foo = HashSet::new();
            Option::Some(Group { members: foo })
        }
    }

    struct Run {
        // a set of three or more consecutive numbers all in the same color.
        // The number 1 is always played as the lowest number, it cannot follow the number 13.
        members: Vec<Tile>, // TODO, Consider a LinkedList
    }

    // #[derive(PartialEq)] // TODO interesting behavior of Vectors
    #[derive(Debug)]
    struct Boneyard {
        // TODO, new constructor here is important. Perhaps a map? instead?
        pub bones: Vec<Tile>,
    }

    impl Boneyard {
        fn new_game() -> Boneyard {
            let mut tiles = vec![JokersWild, JokersWild]; // tradeoffs vs push push?
            for color in Color::iter() {
                for num in Number::iter() {
                    tiles.push(RegularTile(color, num));
                    tiles.push(RegularTile(color, num));
                }
            }
            Boneyard { bones: tiles }
        }

        fn draw_one() -> (Tile, Boneyard) {
            // TODO here, but with the idea of immutablity, when drawing, we get a whole new boneyard
            (
                RegularTile(Color::Red, Number::Twelve),
                Boneyard { bones: Vec::new() },
            )
        }
    }

    struct PlayerRack {
        tiles: Vec<Tile>,
    }

    struct Table {
        // Publicly viewable and mutable by all players, has all the sets that have been placed
        sets: Vec<Set>,
    }

    //Whoa! Unit tests go IN THE SAME FILE
    #[cfg(test)]
    mod tests {
        use crate::rummikub_domain::types::{Color, Number, Tile};

        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }

        #[test]
        fn print_some_types() {
            let color: Color = Color::Black;
            assert_eq!(color, Color::Black);
            println!("Dingus {:?}", color);
            let tile: Tile = Tile::RegularTile(Color::Black, Number::One);
            // Todo iterate throught a list of enums
            println!("How bout a tile: {:?}", tile)
        }
    }

    #[cfg(test)]
    mod test_boneyard {

        use crate::rummikub_domain::types::{Boneyard, Color, Number, Tile};
        use std::ops::Deref;
        use strum::IntoEnumIterator;

        ///106 tiles (8 sets of tiles 1-13 in four colours (2 of each), and 2 joker tiles)
        #[test]
        fn verify_initial_state() {
            let mut state = Boneyard::new_game();
            let bones = state.bones; // Butterfly Meme: Is this a reference? Or a copy? -> No! It's a MOVE!
            assert_eq!(bones.len(), 106);
            for c in Color::iter() {
                let count = bones.iter().filter(|t| is_color(t, c)).count();
                assert_eq!(count, 26);
                // What about implementation version
                let count2 = bones.iter().filter(|t| t.is_color(c)).count();
                assert_eq!(count2, 26);
            }
            for i in Number::iter() {
                let count = bones.iter().filter(|t| t.is_number(i)).count();
                assert_eq!(count, 8);
            }
            let jokers = bones.iter().filter(|t| is_joker(t)).count();
            assert_eq!(jokers, 2);

            /// if let expression crunches brain. For learning, English words for this terse line
            /// Using the abbreviated from of the exhaustive match expression, check this tile input,
            /// which happens to be a composite enum, such that only succeed in the condition where
            /// it's color matches our condition (originally blue but now runtime),
            /// Then since it matches, execute the following expression (statement? block? -> *terminology*)
            /// since this case for us is just boolean, return true, otherwise which since the match statement
            /// is using the terse form representing ALL OTHER possible states, return false.
            /// this is just a predicate. however, it can AND SHOULD be an impl behavior of tile! terse?
            /// Also this is apparently local to the test module
            fn is_color(t: &Tile, color: Color) -> bool {
                // apparently doesn't work, because we are actually just assiging the thing of the tile to `color` not using input
                // if let Tile::RegularTile(color, _) = t { true } else { false }
                if let Tile::RegularTile(c, _) = t {
                    return *c == color;
                }
                false
            }
            fn is_joker(t: &Tile) -> bool {
                if let Tile::JokersWild = t {
                    true
                } else {
                    false
                }
            }
        }
    }
}
