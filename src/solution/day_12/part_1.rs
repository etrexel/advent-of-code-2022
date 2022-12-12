// TODO: docs

use crate::solution::day_12::Map;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let terrain = Map::new(input)?;
    Ok(terrain.shortest_path_length()?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!("31", solve(input).expect("should return result"));
    }
}
