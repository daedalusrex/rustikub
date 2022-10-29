
#[allow(dead_code)]
pub mod types {

    //106 tiles in the game, including 104 numbered tiles (valued 1 to 13 in four different colors, two copies of each) and two jokers

    use std::collections::HashSet;
    // TODO , I don't actually know what to do with these but good example for crates
    use strum_macros::EnumString;
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;
    use crate::rummikub_domain::types::Tile::{RegularTile, JokersWild};

    #[derive(EnumIter, Debug, PartialEq, Eq, Hash, Copy, Clone)]
    enum Color {
        Red,
        Blue,
        Orange,
        Black,
        // AnyColor, // TODO Should this exist? Thinking about monoids and or Wilds
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
        JokersWild  // Todo, do they have a color?
    }

    enum Set {
        // There are two kinds of sets, either a group or a run
        Group,
        Run
    }

    struct Group {
        //A set of either three or four tiles of the same number in different colors.
        members: HashSet<Tile>
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
        members: Vec<Tile> // TODO, Consider a LinkedList
    }


    // #[derive(PartialEq)] // TODO interesting behavior of Vectors
    #[derive(Debug)]
    struct Boneyard {
        // TODO, new constructor here is important. Perhaps a map? instead?
        pub bones: Vec<Tile>
    }

    impl Boneyard {
        // TODO, FYI, this new keyword thing is not actually special for Rust, probs better to call it like new game or init
        fn new() -> Option<Boneyard> {
            let mut tiles: Vec<Tile> = Vec::new();
            // tiles.push(JokersWild);
            // tiles.push(JokersWild);
            tiles.append(&mut vec![JokersWild, JokersWild]);

            for color in Color::iter() {
                for num in Number::iter() {
                    println!("Look a Tile!: {:?}", RegularTile(color, num))
                }

            }
            return Some(Boneyard { bones: tiles})
        }

        fn draw_one() -> (Tile, Boneyard) {
            // TODO here, but with the idea of immutablity, when drawing, we get a whole new boneyard
            (RegularTile(Color::Red, Number::Twelve), Boneyard { bones: Vec::new()})
        }
    }

    struct PlayerRack {
        tiles: Vec<Tile>
    }

    struct Table {
        // Publicly viewable and mutable by all players, has all the sets that have been placed
        sets: Vec<Set>
    }

    //Whoa! Unit tests go IN THE SAME FILE
    #[cfg(test)]
    mod tests {
        use crate::rummikub_domain::types::{Color, Number, Tile};
        use crate::rummikub_domain::types::Color::Black;

        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }


        #[test]
        fn print_some_types() {
            let color: Color = Black;
            assert_eq!(color, Color::Black);
            println!("Dingus {:?}", color);
            let tile: Tile = Tile::RegularTile(Color::Black, Number::One);
            // TOdo iterate throught a list of enums
            println!("How bout a tile: {:?}", tile)

        }
    }


    #[cfg(test)]
    mod test_boneyard {
        use std::borrow::Borrow;
        use std::ops::Deref;
        use crate::rummikub_domain::types::{Boneyard, Color, Tile};

        #[test]
        fn build_new_boneyard() {
            let mut bones = Boneyard::new();
            // The fancy if let construct
            if let Some(initial) = bones {
                //106 tiles (8 sets of tiles 1-13 in four colours, and 2 joker tiles)
                assert_eq!(initial.bones.len(), 106);
                let foo = initial.bones; // Butterfly Meme: Is this a reference? Or a copy?
                let bar = foo.iter().filter(|&x1| {
                    match x1 {
                        Tile::RegularTile(Color::Red, _) => true,
                        Tile::JokersWild => false,
                        _ => false,
                    }
                });

                /// if let expression crunches brain. For learning, English words for this terse line
                /// Using the abbreviated from of the exhaustive match expression, check this tile input,
                /// which happens to be a composite enum, such that only succeed in the condition where it's color matches our condition (blue),
                /// Then since it matches, execute the following expression (statement? block? -> *terminology*)
                /// since this case for us is just boolean, return true, otherwise which since the match statement
                /// is using the terse form representing ALL OTHER possible states, return false.
                /// this is just a predicate.
                fn is_blue(t: &Tile) -> bool {
                    if let Tile::RegularTile(Color::Blue, _) = t { true } else { false }
                }

                let bazing = foo.iter().filter(|&x| {is_blue(x)});
                println!("Expect 26, Found Red Count: {:?}",bar.count());

            }



        }

    }
}
