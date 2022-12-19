use clap:: {
    Args,
    Parser,
    Subcommand
};

/// Source for information on various orbital class rockets currently (or very recently) in production.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct RocketLineArgs {
    /// Name of the rocket you are interested in
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}