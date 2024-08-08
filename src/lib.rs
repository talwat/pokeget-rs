use rust_embed::RustEmbed;

pub mod cli;
pub mod pokemon;
pub mod sprites;

#[derive(RustEmbed)]
#[folder = "data/pokesprite/pokemon-gen8"]
pub struct Data;
