use anyhow::{anyhow, Context};
use std::fs;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

type Solver = fn(&str) -> Result<String, anyhow::Error>;

pub fn solve(day: u8, part: u8, file: Option<String>) -> Result<String, anyhow::Error> {
    let fn_list: Vec<Vec<Solver>> = vec![
        vec![day_01::part_1::solve, day_01::part_2::solve],
        vec![day_02::part_1::solve, day_02::part_2::solve],
        vec![day_03::part_1::solve, day_03::part_2::solve],
        vec![day_04::part_1::solve, day_04::part_2::solve],
        vec![day_05::part_1::solve, day_05::part_2::solve],
        vec![day_06::part_1::solve, day_06::part_2::solve],
        vec![day_07::part_1::solve, day_07::part_2::solve],
    ];
    if !(1..=7).contains(&day) {
        return Err(anyhow!("invalid day: {}", day));
    }
    if !(1..=2).contains(&part) {
        return Err(anyhow!("invalid part: {}", part));
    }
    let file_path = get_file_path(day, part, file)?;
    let contents = read_file_to_string(&file_path)?;
    fn_list[day as usize - 1][part as usize - 1](&contents)
}

fn get_file_path(day: u8, _: u8, file: Option<String>) -> Result<String, anyhow::Error> {
    if let Some(existing_path) = file {
        return Ok(existing_path);
    }
    Ok(format!("input/day_{:02}/input.txt", day))
}

fn read_file_to_string(file_path: &str) -> Result<String, anyhow::Error> {
    fs::read_to_string(&file_path)
        .with_context(|| format!("could not read input file: {}", &file_path))
}
