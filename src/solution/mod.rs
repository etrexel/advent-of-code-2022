//! Solvers for each day and part.
//!
//! Each day is separated into its own module containing the solver for each part and any
//! day-specific helpers or structs. All solvers are exposed via a top-level [solve] function.

use anyhow::{anyhow, Context};
use std::fs;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;

/// Type alias for the function signature that each solver uses.
type Solver = fn(&str) -> Result<String, anyhow::Error>;

/// Executes the selected day and part's solver using either the provided input or default path.
///
/// Each solver is implemented to process the given day and part's input exactly as given by the
/// [Advent of Code](https://adventofcode.com/) website, so you should not need to make any
/// alterations.
///
/// An error is returned if there is any issue with the input such as the input file not existing,
/// an error reading the input file, or an error reading
///
/// # Examples
///
/// ```
/// // executes the solver for the first part of the first day using the default file path
/// let answer = aoc::solve(1, 1, None).unwrap();
/// assert_eq!("69626", answer);
/// ```
pub fn solve(day: u8, part: u8, file: Option<String>) -> Result<String, anyhow::Error> {
    // list of each day and part's solvers
    let fn_list: Vec<Vec<Solver>> = vec![
        vec![day_01::part_1::solve, day_01::part_2::solve],
        vec![day_02::part_1::solve, day_02::part_2::solve],
        vec![day_03::part_1::solve, day_03::part_2::solve],
        vec![day_04::part_1::solve, day_04::part_2::solve],
        vec![day_05::part_1::solve, day_05::part_2::solve],
        vec![day_06::part_1::solve, day_06::part_2::solve],
        vec![day_07::part_1::solve, day_07::part_2::solve],
        vec![day_08::part_1::solve, day_08::part_2::solve],
        vec![day_09::part_1::solve, day_09::part_2::solve],
        vec![day_10::part_1::solve, day_10::part_2::solve],
        vec![day_11::part_1::solve, day_11::part_2::solve],
        vec![day_12::part_1::solve, day_12::part_2::solve],
        vec![day_13::part_1::solve, day_13::part_2::solve],
        vec![day_14::part_1::solve, day_14::part_2::solve],
    ];
    // check that the provided day has an associated function set from the fn_list
    if !(1..=14).contains(&day) {
        return Err(anyhow!("invalid day: {}", day));
    }
    // each day has only part 1 or part 2
    if !(1..=2).contains(&part) {
        return Err(anyhow!("invalid part: {}", part));
    }
    // resolve file path
    let file_path = get_file_path(day, part, file)?;
    // read file
    let contents = read_file_to_string(&file_path)?;
    // execute selected solver
    fn_list[day as usize - 1][part as usize - 1](&contents)
}

/// Resolves the file path for the input data.
///
/// This helper either returns the file path that was provided, or builds a file path based on the
/// default structure: `input/day_XX/input.txt`. It is assumed that the `input` directory exists in
/// the current working directory.
///
/// `XX` is a padded integer representation of the selected day (e.g. `01`, `06`, `11`).
fn get_file_path(day: u8, _: u8, file: Option<String>) -> Result<String, anyhow::Error> {
    // if the user provided a file path just return that
    if let Some(existing_path) = file {
        return Ok(existing_path);
    }
    // build the file path based on the default directory structure
    Ok(format!("input/day_{:02}/input.txt", day))
}

/// Reads the input file into a string or returns an error if there is an issue reading the file.
fn read_file_to_string(file_path: &str) -> Result<String, anyhow::Error> {
    fs::read_to_string(&file_path)
        .with_context(|| format!("could not read input file: {}", &file_path))
}
