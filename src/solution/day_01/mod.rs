//! Solvers for day 01.
//!
//! The parts for day 01 focus on groups of numbers that represent items belonging to individuals.
//! Each part requires summing each individual group and then doing something with the sums. The
//! [parse_input] function is used by both parts to turn the input into the required list of sums.

use anyhow::Context;

pub(crate) mod part_1;
pub(crate) mod part_2;

/// Sum number groups separated by newlines.
///
/// The input should be a string containing groups of whole numbers separated by newlines.
///
/// Returns an error if any tokens from the input cannot be parsed into a u32.
fn parse_input(input: &str) -> Result<Vec<u32>, anyhow::Error> {
    let mut output = Vec::<u32>::new();
    output.push(0);
    let mut idx = 0;
    for line in input.split('\n') {
        // if we have hit the end of a group create the next group
        if line.is_empty() {
            output.push(0);
            idx += 1;
        } else {
            // parse line to u32 and add to group total
            output[idx] += line.parse::<u32>().context("could not parse line to u32")?;
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(
            vec!(6000, 4000, 11000, 24000, 10000),
            parse_input(input).expect("should return vec")
        );
    }
}
