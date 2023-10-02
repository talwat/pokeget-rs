//! Display pokemon sprites in your terminal.

#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery
)]

use clap::Parser;
use inflector::Inflector;
use pokeget::cli::Args;
use pokeget::sprites::{combine_sprites, get_sprites};
use pokeget::utils::get_form;
use pokeget::Data;
use std::process::exit;

fn format_name(name: &String) -> String {
    name.to_title_case().replace('-', " ")
}

fn main() {
    let file = Data::get("pokemon.txt").unwrap();
    let raw = core::str::from_utf8(file.data.as_ref()).unwrap();
    let list: Vec<&str> = raw.split('\n').collect();

    let args = Args::parse();

    if args.pokemon.is_empty() {
        eprintln!("you must specify the pokemon you want to display");
        exit(1);
    }

    let form = get_form(&args);

    let mut pokemons = args.pokemon;

    let (width, height, sprites) =
        get_sprites(&mut pokemons, args.shiny, args.female, &form, &list);
    let combined = combine_sprites(width, height, &sprites);

    if !args.hide_name {
        eprintln!(
            "{}\n",
            pokemons
                .iter()
                .enumerate()
                .map(|(i, x)| format_name(x) + if i != pokemons.len() - 1 { ", " } else { "" })
                .collect::<String>()
        );
    }

    println!("{}", showie::to_ascii(&combined));
}
