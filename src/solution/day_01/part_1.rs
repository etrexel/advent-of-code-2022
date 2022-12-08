//! Solver for part 1.

use crate::solution::day_01::parse_input;

/// Sum all individual groups separated by newlines and return the max of the groups.
///
/// Uses [parse_input] for day 01 to turn the input into a list of sums, then finds the maximum
/// sum in the group. If no maximum is found `0` is returned.
pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    // get list of sums
    let cal_counts = parse_input(input)?;
    // find max or return 0
    Ok(cal_counts.iter().max().unwrap_or(&0).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
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
        assert_eq!("24000", solve(input).expect("should return string result"));
    }
}
