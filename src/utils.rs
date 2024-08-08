use rand::Rng;

use crate::cli::Args;

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
