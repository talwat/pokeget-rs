//! Display pokemon sprites in your terminal.

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]

use clap::Parser;
use pokeget::cli::Args;
use pokeget::pokemon::{Attributes, Pokemon};
use pokeget::sprites::combine_sprites;
use std::process::exit;

fn main() {
    let pokemon_list: Box<[&'static str]> =
        include_str!("../data/pokemon.txt").split('\n').collect();

    let args = Args::parse();

    if args.pokemon.is_empty() {
        eprintln!("you must specify the pokemon you want to display");
        exit(1);
    }

    let attributes = Attributes::new(&args);
    let pokemons: Vec<Pokemon> = args.pokemon
        .into_iter()
        .map(|x| Pokemon::new(x, &pokemon_list, &attributes))
        .collect();

    let combined = combine_sprites(&pokemons);

    if !args.hide_name {
        let names: Vec<&str> = pokemons.iter().map(|x| x.name.as_ref()).collect();

        eprintln!("{}", names.join(", "));
    }

    println!("{}", showie::to_ascii(&combined));
}
