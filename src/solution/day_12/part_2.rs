// TODO: docs

use crate::solution::day_12::Map;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let terrain = Map::new(input)?;
    Ok(terrain.shortest_path_length_from_base()?.to_string())
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
        assert_eq!("29", solve(input).expect("should return result"));
    }
}
