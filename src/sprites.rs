use std::process::exit;

use image::{DynamicImage, GenericImage};

use crate::{utils::random, Data};

/// Fetches a sprite and returns a vector of bytes.
/// This will also format the names properly.
pub fn get_sprite(
    pokemon: &mut String,
    form: &String,
    shiny: bool,
    female: bool,
    gen7: bool,
    list: &[&str],
) -> Vec<u8> {
    if let Ok(pokedex_id) = pokemon.parse::<usize>() {
        if pokedex_id == 0 {
            *pokemon = String::from("random");
        } else if list.len() >= pokedex_id {
            *pokemon = String::from(list[pokedex_id - 1]);
        }
    }

    let is_random = pokemon == "random";

    if is_random {
        *pokemon = random(list);
    }

    let mut filename = pokemon.to_owned();

    // The form shouldn't be applied to random pokemon.
    if !form.is_empty() && !is_random {
        filename.push('-');
        filename.push_str(form);
    }

    // I hate Mr. Mime and Farfetch'd.
    filename = filename
        .replace([' ', '_'], "-")
        .replace(['.', '\'', ':'], "")
        .to_lowercase();

    let mut path = String::new();

    // Try gen7x first then fall back to gen8
    for gen in IntoIterator::into_iter(["gen7x", "gen8"]) {

        // Skip gen7x folder if checking gen7 is disabled
        if !gen7 && gen == "gen7x" {
            continue;
        }

        let m_path = &format!(
            "pokesprite/pokemon-{}/{}/{}{filename}.png",
            gen,
            if shiny { "shiny" } else { "regular" },
            if female && !is_random { "female/" } else { "" } // Random pokemon also shouldn't follow the female rule.
        );

        // If found a valid sprite, set the path
        if Data::get(m_path).is_some() {
            path = m_path.to_owned();
            break;
        }
    }

    Data::get(path.as_str())
        .unwrap_or_else(|| {
            eprintln!("pokemon not found");
            exit(1);
        })
        .data
        .into_owned()
}

/// Combines several sprites into one by stitching them horizontally.
pub fn combine_sprites(
    combined_width: u32,
    combined_height: u32,
    sprites: &[DynamicImage],
) -> DynamicImage {
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
/// This will also format the names properly.
///
/// Mutable access to `pokemons` is required to edit the names of random pokemon so they can be displayed.
pub fn get_sprites(
    pokemons: &mut [String],
    shiny: bool,
    female: bool,
    gen7: bool,
    form: &String,
    list: &[&str],
) -> (u32, u32, Vec<DynamicImage>) {
    let mut sprites = Vec::new();
    let mut combined_width: u32 = 0;
    let mut combined_height: u32 = 0;

    for pokemon in pokemons.iter_mut() {
        let bytes = get_sprite(pokemon, form, shiny, female, gen7, list);

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
