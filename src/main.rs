use std::{process::exit, fs};

use clap::Parser;
use rand::Rng;
use reqwest::StatusCode;

// Display pokemon sprites in your terminal.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pokemon to display, use "random" to get a random pokemon
    pokemon: String,

    /// The form of the pokemon
    #[arg(short, long, default_value="")]
    form: String,

    /// Display the pokemon as it's mega form
    #[arg(short, long, default_value_t=false)]
    mega: bool,

    /// Display the pokemon as it's mega X form
    #[arg(long, default_value_t=false)]
    mega_x: bool,

    /// Display the pokemon as it's mega Y form
    #[arg(long, default_value_t=false)]
    mega_y: bool,

    /// Display the pokemon as shiny
    #[arg(short, long, default_value_t=false)]
    shiny: bool,

    /// Display the alolan variant of the pokemon
    #[arg(short, long, default_value_t=false)]
    alolan: bool,
}

fn get_sprite(pokemon: &String, form: &String, shiny: bool) -> bytes::Bytes {
    let mut filename = pokemon.clone();
    if form != "" {
        filename.push('-');
        filename.push_str(&form);
    }

    // I hate Mr. Mime and Farfetch'd.
    filename = filename
        .replace(" ", "-")
        .replace("_", "-")
        .replace(".", "")
        .replace("'", "")
        .replace(":", "")
        .to_lowercase();

    let resp = reqwest::blocking::get(format!("https://raw.githubusercontent.com/msikma/pokesprite/master/pokemon-gen8/{}/{filename}.png", if shiny { "shiny" } else { "regular" })).unwrap();
    let status = resp.status();
    
    if status == StatusCode::NOT_FOUND {
        eprintln!("pokemon not found");
        exit(1);
    } else if status != StatusCode::OK {
        eprintln!("http error {}", status)
    }

    return resp.bytes().unwrap();
}

fn random() -> String {
    let contents = fs::read_to_string("./data/pokemon.txt")
        .expect("Should have been able to read the file");

    let pokemons: Vec<&str> = contents.split('\n').collect();

    let mut rand = rand::thread_rng();

    return String::from(pokemons[rand.gen_range(0..pokemons.len())]);
}

fn main() {
    let args = Args::parse();

    let pokemon;

    if args.pokemon == "random" {
        let random_pokemon = random();
        eprintln!("{}\n", random_pokemon);

        pokemon = random_pokemon;
    } else {
        pokemon = args.pokemon;
    }

    let form;

    if args.mega {
        form = String::from("mega")
    } else if args.mega_x {
        form = String::from("mega-x")
    } else if args.mega_y {
        form = String::from("mega-y")
    } else if args.alolan {
        form = String::from("alola")
    } else {
        form = args.form;
    }

    let bytes = get_sprite(&pokemon, &form, args.shiny);
    let img = image::load_from_memory(&bytes).unwrap();
    let trimmed = showie::trim(&img);

    println!("{}", showie::to_ascii(&trimmed));
}
