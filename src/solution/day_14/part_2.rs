// TODO: docs

use crate::solution::day_14::Cave;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let mut cave = Cave::new(input)?;
    cave.simulate_sand(true);
    Ok(cave.count_sand().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!("93", solve(input).expect("should return result"));
    }
}
