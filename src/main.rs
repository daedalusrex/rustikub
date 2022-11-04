#![allow(dead_code, unused_imports, unused_variables)] //TODO remove during clean up phase

extern crate core;

mod domain;

fn main() {
    pub use domain::tiles::Tile; // works while using Mod.rs
    println!("Hello, world!");
}
