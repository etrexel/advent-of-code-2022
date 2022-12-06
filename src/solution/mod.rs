use anyhow::{anyhow, Context};
use std::fs;

mod day_one;
mod day_two;

pub fn solve(day: u8, part: u8, file: Option<String>) -> Result<String, anyhow::Error> {
    match day {
        1 => {
            let file_path = file.unwrap_or("input/day_one/input.txt".to_string());
            let contents = read_file_to_string(&file_path)?;
            match part {
                1 => day_one::part_one::solve(&contents),
                2 => day_one::part_two::solve(&contents),
                _ => Err(anyhow!("invalid part selection")),
            }
        }
        2 => {
            let file_path = file.unwrap_or("input/day_two/input.txt".to_string());
            let contents = read_file_to_string(&file_path)?;
            match part {
                1 => day_two::part_one::solve(&contents),
                2 => day_two::part_two::solve(&contents),
                _ => Err(anyhow!("invalid part selection")),
            }
        }
        _ => Err(anyhow!("invalid day selection")),
    }
}

fn read_file_to_string(file_path: &str) -> Result<String, anyhow::Error> {
    fs::read_to_string(&file_path)
        .with_context(|| format!("could not read input file: {}", &file_path))
}
