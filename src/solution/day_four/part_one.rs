use crate::solution::day_four::parse_input;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let mut total = 0;
    let pairs = parse_input(input)?;
    for pair in pairs {
        if pair.0.contains(&pair.1) || pair.1.contains(&pair.0) {
            total += 1
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!("2", solve(input).expect("should return result"));
    }
}
