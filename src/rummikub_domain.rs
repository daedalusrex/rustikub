pub mod types {
    enum Color {
        Red,
        Blue,
        Orange,
        Black,
        Colorless, // TODO Should this exist? Thinking about monoids
    }

    enum Cardinality {
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
        // Maybe later I can add some traits or ordering or something
    }
}
