use std::process::exit;

use image::DynamicImage;

use rand::Rng;

use crate::{cli::Args, list::List, Data};

const DEFAULT_SHINY_RATE: u32 = 8192;

/// Enum used to store each region
#[derive(PartialEq, Eq)]
pub enum Region {
    Kanto,
    Johto,
    Hoenn,
    Sinnoh,
    Unova,
    Kalos,
    Alola,
    Galar,
}

/// Enum used to assist parsing user input.
///
/// It can sort all types of inputs, and then evaluate them to a filename.
#[derive(PartialEq, Eq)]
pub enum Selection {
    /// When a random pokemon is selected (`0` or `random`).
    Random,

    /// When a region is selected (e.g Kanto)
    Region(Region),

    /// When a DexID is selected (number larger than 0).
    DexId(usize),

    /// When a pokemon name/id is selected.
    Name(String),
}

impl Selection {
    /// Parses a raw argument into a [`Selection`].
    pub fn parse(arg: String) -> Self {
        if let Ok(dex_id) = arg.parse::<usize>() {
            match dex_id {
                // If it's zero, then change it to random.
                0 => Selection::Random,

                // If it's not zero and in the range of the list, then it's a dex id.
                id if (id > 0) => Selection::DexId(id - 1),

                // This shouldn't normally fire, but it's here to give the proper error message.
                _ => Selection::Name(arg),
            }
        } else {
            match arg.to_lowercase().as_str() {
                "random" => Selection::Random,
                "kanto" => Selection::Region(Region::Kanto),
                "johto" => Selection::Region(Region::Johto),
                "hoenn" => Selection::Region(Region::Hoenn),
                "sinnoh" => Selection::Region(Region::Sinnoh),
                "unova" => Selection::Region(Region::Unova),
                "kalos" => Selection::Region(Region::Kalos),
                "alola" => Selection::Region(Region::Alola),
                "galar" => Selection::Region(Region::Galar),
                _ => Selection::Name(arg),
            }
        }
    }

    /// Evaluates the selection and returns a pokemon filename.
    pub fn eval(self, list: &List) -> String {
        match self {
            Selection::Random => list.random(),
            Selection::Region(region) => list.get_by_region(region),
            Selection::DexId(id) => list
                .get_by_id(id)
                .unwrap_or_else(|| {
                    // add 1 to id so that error message matches user input
                    eprintln!("{} is not a valid pokedex ID", id + 1);
                    exit(1)
                })
                .clone(),
            Selection::Name(name) => name,
        }
    }
}

/// The struct used to represent a Pokemon's data.
/// This includes it's file path, formatted name, sprite, and attributes.
pub struct Pokemon<'a> {
    /// The path of the Pokemon in pokesprite.
    /// Eg. `regular/abra.png`
    pub path: String,

    /// The formatted name of the pokemon, usually gotten from a [List].
    pub name: String,

    /// The sprite of the Pokemon, as a [DynamicImage].
    pub sprite: DynamicImage,

    /// Data, like the form and whether a pokemon is shiny or not.
    pub attributes: &'a Attributes,
}

impl<'a> Pokemon<'a> {
    /// Creates a new pokemon.
    /// This also fetches the sprite & formats the name.
    pub fn new(arg: String, list: &List, attributes: &'a Attributes) -> Self {
        let selection = Selection::parse(arg);
        let is_random = selection == Selection::Random;
        let is_region = matches!(selection, Selection::Region(_));
        let name = selection.eval(list);

        let path = attributes.path(&name, is_random, is_region);
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
            name: list.format_name(&name),
            sprite: trimmed,
            attributes,
        }
    }
}

/// Handles parsing the form, as well as whether a pokemon is female or shiny.
pub struct Attributes {
    pub form: String,
    pub female: bool,
    pub shiny: bool,
}

/// Pokemon attribues, like whether it's shiny, female, and it's form.
impl Attributes {
    /// Determine whether a pokemon should be shiny, based on a random rate (`DEFAULT_SHINY_RATE`).
    ///
    /// If the user specified that they want a shiny pokemon, then this function is irrelevant.
    fn rate_is_shiny() -> bool {
        let rate = match std::env::var("POKEGET_SHINY_RATE")
            .map_err(|_| false)
            .and_then(|x| x.parse::<u32>().map_err(|_| true))
        {
            Ok(rate) => rate.max(1), // No zero please
            Err(should_notify) => {
                if should_notify {
                    eprintln!("POKEGET_SHINY_RATE was improperly formatted, using default rate")
                }

                DEFAULT_SHINY_RATE
            }
        };

        0 == rand::thread_rng().gen_range(0..rate)
    }
    /// Make a new [`Attributes`] by parsing the command line arguments.
    pub fn new(args: &Args) -> Self {
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

        Self {
            form,
            female: args.female,
            shiny: args.shiny || Self::rate_is_shiny(),
        }
    }

    /// Formats the attributes and a filename from a [Selection] into a completed path.
    pub fn path(&self, name: &str, random: bool, region: bool) -> String {
        let mut filename = name.to_owned();

        // The form shouldn't be applied to random or region pokemon.
        let is_random = random || region;
        if !self.form.is_empty() && !is_random {
            filename.push_str(&format!("-{}", self.form));
        }

        // I hate Mr. Mime and Farfetch'd.
        filename = filename
            .replace([' ', '_'], "-")
            .replace(['.', '\'', ':'], "")
            .to_lowercase();

        let path = format!(
            "{}/{}{}.png",
            if self.shiny { "shiny" } else { "regular" },
            if self.female && !is_random {
                "female/"
            } else {
                ""
            }, // Random or region pokemon also shouldn't follow the female rule.
            filename.trim()
        );

        path
    }
}
