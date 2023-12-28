use rust_embed::RustEmbed;

pub mod cli;
pub mod sprites;
pub mod utils;

#[derive(RustEmbed)]
#[folder = "data/"]
#[include = "pokesprite/pokemon-gen8/*"]
#[include = "pokesprite/pokemon-gen7x/*"]
#[include = "pokemon.txt"]
pub struct Data;
