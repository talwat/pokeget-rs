use rust_embed::RustEmbed;

pub mod sprites;
pub mod cli;
pub mod utils;

#[derive(RustEmbed)]
#[folder = "data/"]
#[include = "pokesprite/pokemon-gen8/*"]
#[include = "pokemon.txt"]
pub struct Data;