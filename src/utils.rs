use rand::Rng;

use crate::cli::Args;

/// Returns a random pokemon.
pub fn random(list: &[&str]) -> String {
    let mut rand = rand::thread_rng();

    String::from(list[rand.gen_range(0..list.len())])
}

/// Uses the arguments like gmax, mega, etc... to get a form which is appended to the pokemon filename.
pub fn get_form(args: &Args) -> String {
    let mut form;

    if args.mega {
        form = String::from("mega");
    } else if args.mega_x {
        form = String::from("mega-x");
    } else if args.mega_y {
        form = String::from("mega-y");
    } else if args.alolan {
        form = String::from("alola");
    } else if args.gmax {
        form = String::from("gmax");
    } else if args.hisui {
        form = String::from("hisui");
    } else if args.galar {
        form = String::from("galar");
    } else {
        form = args.form.clone();
    }

    if args.noble {
        form.push_str("-noble");
    }

    form
}
