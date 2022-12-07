use crate::solution::day_01::parse_input;

/// Sum all the individual groups separated by newlines and return the sum of the top three groups
pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let mut cal_counts = parse_input(input)?;
    cal_counts.sort();
    cal_counts.reverse();
    let res = match cal_counts.len() {
        0 => 0,
        1 | 2 | 3 => cal_counts.iter().sum(),
        _ => cal_counts[0] + cal_counts[1] + cal_counts[2],
    };
    Ok(res.to_string())
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
        assert_eq!("45000", solve(input).expect("should return string result"));
    }
}
