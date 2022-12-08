//! Solver for part 2.

use crate::solution::day_08::TreeGrid;

/// Find the max scenic score of trees.
///
/// Uses the [TreeGrid] to parse the input and calculate the maximum scenic score of all the trees.
pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let tree_grid = TreeGrid::new(input)?;
    Ok(tree_grid.max_scenic_score().to_string())
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
        assert_eq!("8", solve(input).expect("should return result"));
    }
}
