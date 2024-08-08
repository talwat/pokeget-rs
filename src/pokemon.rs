use std::process::exit;

use image::DynamicImage;
use inflector::Inflector;
use rand::Rng;

use crate::{cli::Args, Data};

/// Returns a random pokemon.
pub fn random(list: &[&str]) -> String {
    let mut rand = rand::thread_rng();

    String::from(list[rand.gen_range(0..list.len())])
}

/// Uses the arguments like gmax, mega, etc... to get a form which is appended to the pokemon filename.
pub fn get_form(args: &Args) -> String {
    let mut form = match args {
        Args { mega: true, .. } => "mega",
        Args { mega_x: true, .. } => "mega-x",
        Args { mega_y: true, .. } => "mega-y",
        Args { alolan: true, .. } => "alola",
        Args { gmax: true, .. } => "gmax",
        Args { hisui: true, .. } => "hisui",
        Args { galar: true, .. } => "galar",
        _ => &args.form,
    }
    .to_string();

    if args.noble {
        form.push_str("-noble");
    }

    form
}

fn format_path(name: &str, form: String, random: bool, shiny: bool, female: bool) -> String {
    let mut filename = name.to_owned();

    // The form shouldn't be applied to random pokemon.
    if !form.is_empty() && !random {
        filename.push_str(&format!("-{}", form));
    }

    // I hate Mr. Mime and Farfetch'd.
    filename = filename
        .replace([' ', '_'], "-")
        .replace(['.', '\'', ':'], "")
        .to_lowercase();

    let path = format!(
        "{}/{}{}.png",
        if shiny { "shiny" } else { "regular" },
        if female && !random { "female/" } else { "" }, // Random pokemon also shouldn't follow the female rule.
        filename.trim()
    );

    return path;
}

fn format_name(name: String) -> String {
    name.replace('-', " ").replace('\'', "").to_title_case()
}

#[derive(PartialEq, Eq)]
pub enum Selection {
    Random,
    DexId(usize),
    Name(String),
}

impl Selection {
    pub fn parse(id: String, list: &[&'static str]) -> Self {
        if let Ok(dex_id) = id.parse::<usize>() {
            return match dex_id {
                // If it's zero, then change it to random.
                0 => Selection::Random,

                //
                id if (id > 0 && id <= list.len()) => Selection::DexId(id - 1),
                _ => Selection::Name(id),
            };
        } else {
            return match id.as_str() {
                "random" => Selection::Random,
                _ => Selection::Name(id),
            };
        }
    }
}

pub struct Pokemon {
    /// The path of the Pokemon in pokesprite.
    /// Eg. `regular/abra.png`
    pub path: String,

    /// The formatted name of the pokemon.
    /// This is usually done by first getting the file path, and then formatting it.
    ///
    /// It's not a perfect solution, but it's the fastest option.
    // TODO: Make a script to use pokesprites' own database and compile a list of
    // filenames and their corrosponding formatted names.
    pub name: String,

    /// The sprite of the Pokemon, as a [DynamicImage].
    pub sprite: DynamicImage,
}

impl Pokemon {
    pub fn new(id: String, list: &[&'static str], form: String, args: &Args) -> Self {
        let mut selection = Selection::parse(id, list);
        let is_random = selection == Selection::Random;

        // If it's random, then let's just look up a random pokemon from the list.
        if is_random {
            selection = Selection::Name(random(list));
        }

        // If it's a dex id, then let's look up that id in the list.
        if let Selection::DexId(dex_id) = selection {
            selection = Selection::Name(list[dex_id].to_string());
        }

        // We've now converted both other types into `Selection::Name`,
        // but if we failed somewhere, then we need to panic.
        let Selection::Name(name) = selection else {
            panic!("selection should have been converted, but wasn't")
        };

        let path = format_path(&name, form, is_random, args.shiny, args.female);
        let bytes = Data::get(&path)
            .unwrap_or_else(|| {
                eprintln!("pokemon not found");
                exit(1)
            })
            .data
            .into_owned();

        let img = image::load_from_memory(&bytes).unwrap();
        let trimmed = showie::trim(&img);

        Self {
            path,
            name: format_name(name),
            sprite: trimmed,
        }
    }
}
