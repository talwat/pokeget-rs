use std::process::exit;

use image::DynamicImage;

use crate::{cli::Args, list::List, Data};

#[derive(PartialEq, Eq)]
pub enum Selection {
    Random,
    DexId(usize),
    Name(String),
    Final(String, String),
}

impl Selection {
    pub fn parse(id: String) -> Self {
        if let Ok(dex_id) = id.parse::<usize>() {
            return match dex_id {
                // If it's zero, then change it to random.
                0 => Selection::Random,

                // If it's not zero and in the range of the list, then it's a dex id.
                id if (id > 0) => Selection::DexId(id - 1),

                // This shouldn't normally fire, but it's here to give the proper error message.
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

pub struct Pokemon<'a> {
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

    /// Data, like the form and whether a pokemon is shiny or not.
    pub attributes: &'a Attributes,
}

pub struct Attributes {
    pub form: String,
    pub female: bool,
    pub shiny: bool,
}

impl Attributes {
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

impl<'a> Pokemon<'a> {
    pub fn new(id: String, list: &List, attributes: &'a Attributes) -> Self {
        let mut selection = Selection::parse(id);
        let is_random = selection == Selection::Random;

        // If it's random, then let's just look up a random pokemon from the list.
        if is_random {
            let random = list.random();
            selection = Selection::Name(random);
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
