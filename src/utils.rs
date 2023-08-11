use rand::Rng;

use crate::cli::Args;

/// Returns a random pokemon.
pub fn random(list: &[&str]) -> String {
    let mut rand = rand::thread_rng();

    String::from(list[rand.gen_range(0..list.len())])
}

/// Uses the arguments like gmax, mega, etc... to get a form which is appended to the pokemon filename.
pub fn get_form(args: &Args) -> String {
    if args.mega {
        String::from("mega")
    } else if args.mega_x {
        String::from("mega-x")
    } else if args.mega_y {
        String::from("mega-y")
    } else if args.alolan {
        String::from("alola")
    } else if args.gmax {
        String::from("gmax")
    } else if args.hisue {
        String::from("hisue")
    } else if args.noble {
        String::from("hisue-noble")
    } else if args.galar {
        String::from("galar")
    } else {
        args.form.clone()
    }
}
