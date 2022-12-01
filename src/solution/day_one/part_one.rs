use crate::solution::day_one::parse_input;

/// Sum all individual groups separated by newlines and return the max of the groups
pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let cal_counts = parse_input(input)?;
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
