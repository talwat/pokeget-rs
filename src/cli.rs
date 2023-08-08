//! Display pokemon sprites in your terminal.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The pokemon to display, use "random" to get a random pokemon
    pub pokemon: Vec<String>,

    /// The form of the pokemon
    #[arg(short, long, default_value = "")]
    pub form: String,

    /// Display the pokemon as it's mega form
    #[arg(short, long, default_value_t = false)]
    pub mega: bool,

    /// Display the pokemon as it's mega X form
    #[arg(long, default_value_t = false)]
    pub mega_x: bool,

    /// Display the pokemon as it's mega Y form
    #[arg(long, default_value_t = false)]
    pub mega_y: bool,

    /// Display the pokemon as shiny
    #[arg(short, long, default_value_t = false)]
    pub shiny: bool,

    /// Display the alolan variant of the pokemon
    #[arg(short, long, default_value_t = false)]
    pub alolan: bool,

    /// Display the gigantamax variant of the pokemon
    #[arg(short, long, default_value_t = false)]
    pub gmax: bool,

    /// Display the hisue variant of the pokemon
    #[arg(long, default_value_t = false)]
    pub hisue: bool,

    /// Display the hisue noble variant of the pokemon
    #[arg(short, long, default_value_t = false)]
    pub noble: bool,
}