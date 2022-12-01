use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Which day's puzzle to solve
    #[arg(short, long, default_value = "1")]
    pub day: u8,

    /// Which part of the day's puzzle to solve
    #[arg(short, long, default_value = "1")]
    pub part: u8,

    /// Path to input file
    #[arg(short, long)]
    pub file: Option<String>,
}

pub(crate) fn parse() -> Cli {
    Cli::parse()
}
