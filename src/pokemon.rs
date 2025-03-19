use std::process::exit;

use image::DynamicImage;

use crate::{cli::Args, list::List, Data};

/// Enum used to assist parsing user input.
///
/// It can sort all types of inputs, and then evaluate them to a filename.
#[derive(PartialEq, Eq)]
pub enum Selection {
    /// When a random pokemon is selected (`0` or `random`).
    Random,

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
            match arg.as_str() {
                "random" => Selection::Random,
                _ => Selection::Name(arg),
            }
        }
    }

    /// Evaluates the selection and returns a pokemon filename.
    pub fn eval(self, list: &List) -> String {
        match self {
            Selection::Random => list.random(),
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
        let name = selection.eval(list);

        let path = attributes.path(&name, is_random);
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
            shiny: args.shiny,
        }
    }

    /// Formats the attributes and a filename from a [Selection] into a completed path.
    pub fn path(&self, name: &str, random: bool) -> String {
        let mut filename = name.to_owned();

        // The form shouldn't be applied to random pokemon.
        if !self.form.is_empty() && !random {
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
            if self.female && !random {
                "female/"
            } else {
                ""
            }, // Random pokemon also shouldn't follow the female rule.
            filename.trim()
        );

        path
    }
}
