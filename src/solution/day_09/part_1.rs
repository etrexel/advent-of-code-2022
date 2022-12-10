//! Solver for part 1.

use crate::solution::day_09::{parse_input, Rope};

/// Count the unique coordinates that the tail of a rope of length 2 visits.
pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    // get movements from input
    let movements = parse_input(input)?;
    let mut rope_bridge = Rope::new(1)?;
    // loop over movements updating rope
    rope_bridge.process_moves(&movements);
    Ok(rope_bridge.tail_visit_count().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!("13", solve(input).expect("should return result"));
    }
}
