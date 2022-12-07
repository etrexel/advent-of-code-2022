use clap::Parser;
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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

fn main() {
    let cli = Cli::parse();
    let result = aoc::solve(cli.day, cli.part, cli.file);
    match result {
        Ok(res) => println!("Day {} part {} solution: {}", cli.day, cli.part, res),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}
