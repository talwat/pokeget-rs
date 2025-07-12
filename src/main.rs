//! Display pokemon sprites in your terminal.

use clap::Parser;
use pokeget::cli::Args;
use pokeget::list::List;
use pokeget::pokemon::{Attributes, Pokemon};
use pokeget::sprites::combine_sprites;
use std::process::exit;

fn main() {
    let list = List::read();
    let args = Args::parse();

    if args.pokemon.is_empty() {
        eprintln!("you must specify the pokemon you want to display");
        exit(1);
    }

    let attributes = Attributes::new(&args);
    let pokemons: Vec<Pokemon> = args
        .pokemon
        .into_iter()
        .map(|x| Pokemon::new(x, &list, &attributes))
        .collect();

    let combined = combine_sprites(&pokemons);

    if !args.hide_name {
        let names: Vec<&str> = pokemons.iter().map(|x| x.name.as_ref()).collect();
        let output = &names.join(", ");
        if args.center {
            let (width, _) = termion::terminal_size().expect("Must be run in terminal");
            let text_len = ansi_width::ansi_width(output);
            let padding = (width as usize - text_len) / 2;
            eprintln!("{:>padding$}{output}", "", padding = padding);
        } else {
            eprintln!("{output}");
        }
    }

    let image = showie::to_ascii(&combined);
    if args.center {
        let (width, _) = termion::terminal_size().expect("Must be run in terminal");
        for line in image.lines() {
            let text_len = ansi_width::ansi_width(line);
            if text_len < width as usize {
                let padding = (width as usize - text_len) / 2;
                println!("{:>padding$}{line}", "", padding = padding);
            } else {
                println!("{line}");
            }
        }
    }
    else {
        println!("{image}");
    }
}
