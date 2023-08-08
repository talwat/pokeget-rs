use std::process::exit;
use clap::Parser;
use pokeget::cli::Args;
use pokeget::Data;
use pokeget::sprites::{get_sprites, combine_sprites};
use pokeget::utils::get_form;

fn main() {
    let file = Data::get("pokemon.txt").unwrap();
    let raw = std::str::from_utf8(file.data.as_ref()).unwrap();
    let list: Vec<&str> = raw.split('\n').collect();

    let args = Args::parse();

    if args.pokemon.is_empty() {
        eprintln!("you must specify the pokemon you want to display");
        exit(1);
    }

    let form = get_form(&args);

    let (width, height, sprites) = get_sprites(&args, &form, &list);
    let combined = combine_sprites(width, height, &sprites);

    println!("{}", showie::to_ascii(&combined));
}
