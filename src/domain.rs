use crate::domain::tiles::Tile;
use std::fmt;
use std::fmt::Formatter;

pub mod player;
pub mod score_value;
pub mod sets;
pub mod table;
pub mod tiles;

// FYI, doing this instead of mod.rs is the 'preferred' convention

#[derive(Debug)]
pub struct RummikubError;

/// Decomposes an abstract group of multiple (or a single) tiles,
/// into the component tiles that constitute the thing that is being decomposed
pub trait Decompose {
    fn decompose(&self) -> Vec<Tile>;
}
