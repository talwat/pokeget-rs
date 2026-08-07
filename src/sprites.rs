use crate::pokemon::Pokemon;
use image::{DynamicImage, GenericImage};

/// Combines several pokemon sprites into a single image
pub fn combine(pokemons: &[Pokemon]) -> DynamicImage {
    let term_width = terminal_size::terminal_size().map_or(u32::MAX, |(w, _)| w.0.into());

    let rows = pack_rows(pokemons, term_width);
    let row_sizes: Vec<(u32, u32)> = rows.iter().map(row_size).collect();

    let total_width = row_sizes.iter().map(|(w, _)| *w).max().unwrap_or(0);
    let total_height: u32 = row_sizes.iter().map(|(_, h)| h).sum();
    let mut canvas = DynamicImage::new_rgba8(total_width, total_height);

    let mut current_y = 0;
    for (row, &(_, row_height)) in rows.iter().zip(&row_sizes) {
        let mut current_x = 0;
        for pokemon in row {
            let sprite = &pokemon.sprite;
            canvas
                .copy_from(sprite, current_x, current_y + row_height - sprite.height())
                .unwrap();
            current_x += sprite.width() + 1;
        }
        current_y += row_height;
    }
    canvas
}

/// Packs pokemon into rows that fit within max_width
fn pack_rows<'a>(pokemons: &'a [Pokemon], max_width: u32) -> Vec<Vec<&'a Pokemon<'a>>> {
    let mut rows: Vec<Vec<&Pokemon>> = vec![vec![]];
    let mut current_width = 0;

    for pokemon in pokemons {
        let w = pokemon.sprite.width() + 1;
        if current_width > 0 && current_width + w > max_width {
            rows.push(vec![]);
            current_width = 0;
        }
        rows.last_mut().unwrap().push(pokemon);
        current_width += w;
    }
    rows
}

fn row_size(row: &Vec<&Pokemon>) -> (u32, u32) {
    let width = row
        .iter()
        .map(|p| p.sprite.width() + 1)
        .sum::<u32>()
        .saturating_sub(1);
    let height = row.iter().map(|p| p.sprite.height()).max().unwrap_or(0);
    (width, height)
}
