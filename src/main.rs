mod cli;
mod solution;

fn main() {
    let cli = cli::parse();
    let result = solution::solve(cli.day, cli.part, cli.file);
    match result {
        Ok(res) => println!("{}", res),
        Err(e) => println!("ERROR: {}", e),
    }
}
