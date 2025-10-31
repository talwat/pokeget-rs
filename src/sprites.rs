use image::{DynamicImage, GenericImage};

use crate::pokemon::Pokemon;

/// Combines several pokemon sprites into one by stitching them horizontally.
pub fn combine(pokemons: &[Pokemon]) -> DynamicImage {
    let mut width: u32 = 0;
    let mut height: u32 = 0;

    for pokemon in pokemons {
        width += pokemon.sprite.width() + 1;
        if pokemon.sprite.height() > height {
            height = pokemon.sprite.height();
        }
    }

    let mut combined = DynamicImage::new_rgba8(width - 1, height);
    let mut shift = 0;

    for pokemon in pokemons {
        combined
            .copy_from(&pokemon.sprite, shift, height - pokemon.sprite.height())
            .unwrap();
        shift += pokemon.sprite.width() + 1;
    }

    combined
}
