use rust_embed::RustEmbed;

pub mod cli;
pub mod sprites;
pub mod utils;

#[derive(RustEmbed)]
#[folder = "data/pokesprite/pokemon-gen8"]
pub struct Data;
