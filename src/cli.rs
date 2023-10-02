use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The pokemon to display, use "random" to get a random pokemon
    pub pokemon: Vec<String>,

    /// Whether to hide the pokemon's name which appears above it
    #[arg(long, default_value_t = false)]
    pub hide_name: bool,

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

    /// Display the hisui variant of the pokemon
    #[arg(long, default_value_t = false)]
    pub hisui: bool,

    /// Display the noble variant of the pokemon, this option often times only works in tandom with --hisui.
    #[arg(short, long, default_value_t = false)]
    pub noble: bool,

    /// Display the galarian variant of the pokemon
    #[arg(long, default_value_t = false)]
    pub galar: bool,

    /// Display the female variant of the pokemon if it exists. This doesn't apply to nidoran, for some reason.
    #[arg(long, default_value_t = false)]
    pub female: bool,
}
