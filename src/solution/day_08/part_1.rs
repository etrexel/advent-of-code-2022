//! Solver for part 1.

use crate::solution::day_08::TreeGrid;

/// Count the total number of trees that are visible from outside the grid.
///
/// Uses the [TreeGrid] to parse the input and calculate the count of visible trees.
pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let tree_grid = TreeGrid::new(input)?;
    Ok(tree_grid.count_visible().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!("21", solve(input).expect("should return result"));
    }
}
