use anyhow::{anyhow, Context};
use std::fs;

mod day_one;

pub(crate) fn solve(day: u8, part: u8, file: Option<String>) -> Result<String, anyhow::Error> {
    match day {
        1 => {
            let file_path = file.unwrap_or("input/day_one/input.txt".to_string());
            let contents = fs::read_to_string(&file_path)
                .with_context(|| format!("could not read input file: {}", &file_path))?;
            match part {
                1 => day_one::part_one::solve(&contents),
                2 => day_one::part_two::solve(&contents),
                _ => Err(anyhow!("invalid part selection")),
            }
        }
        _ => Err(anyhow!("invalid day selection")),
    }
}
