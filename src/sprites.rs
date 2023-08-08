use std::process::exit;

use image::{DynamicImage, GenericImage};

use crate::{cli::Args, utils::random, Data};

/// Fetches a sprite and returns a vector of bytes.
pub fn get_sprite(pokemon: &str, form: &String, shiny: bool) -> Vec<u8> {
    let mut filename = pokemon.to_owned();
    if !form.is_empty() {
        filename.push('-');
        filename.push_str(form);
    }

    // I hate Mr. Mime and Farfetch'd.
    filename = filename
        .replace([' ', '_'], "-")
        .replace(['.', '\'', ':'], "")
        .to_lowercase();

    let path = &format!(
        "pokesprite/pokemon-gen8/{}/{filename}.png",
        if shiny { "shiny" } else { "regular" }
    );

    Data::get(path)
        .unwrap_or_else(|| {
            eprintln!("pokemon not found");
            exit(1);
        })
        .data
        .into_owned()
}

/// Combines several sprites into one by stitching them horizontally.
pub fn combine_sprites(combined_width: u32, combined_height: u32, sprites: &Vec<DynamicImage>) -> DynamicImage {
    let mut combined = DynamicImage::new_rgba8(combined_width - 1, combined_height);
    let mut shift = 0;

    for sprite in sprites {
        combined
            .copy_from(sprite, shift, combined_height - sprite.height())
            .unwrap();
        shift += sprite.width() + 1;
    }

    combined
}

/// Loops through all the pokemon specified in the args and returns a vector of images.
pub fn get_sprites(args: &Args, form: &String, list: &Vec<&str>) -> (u32, u32, Vec<DynamicImage>) {
    let mut sprites = Vec::new();
    let mut combined_width: u32 = 0;
    let mut combined_height: u32 = 0;

    for pokemon in &args.pokemon {
        let bytes = if pokemon == "random" {
            get_sprite(&random(list), &String::new(), args.shiny)
        } else {
            get_sprite(pokemon, form, args.shiny)
        };

        let img = image::load_from_memory(&bytes).unwrap();
        let trimmed = showie::trim(&img);

        combined_width += trimmed.width() + 1;

        if trimmed.height() > combined_height {
            combined_height = trimmed.height();
        }

        sprites.push(trimmed);
    }

    (combined_width, combined_height, sprites)
}