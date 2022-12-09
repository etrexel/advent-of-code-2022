//! Solver for part 2.

use crate::solution::day_09::{parse_input, Rope};

/// Count the unique coordinates that the tail of a rope of length 10 visits.
///
/// The gotcha for this part is that due to the way the knots in the rope get updated it is
/// possible for a longer rope to end up having a knot "jump" such that it is 2 units away on both
/// axes. This case isn't possible for shorter ropes and isn't covered by the basic unit tests.
pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    // get movements from input
    let movements = parse_input(input)?;
    let mut rope_bridge = Rope::new(9)?;
    // loop over movements updating rope
    for movement in movements {
        rope_bridge.process_move(&movement)
    }
    Ok(rope_bridge.tail_visit_count().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!("36", solve(input).expect("should return result"));
    }
}
