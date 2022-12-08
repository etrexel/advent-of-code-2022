//! Solvers for day 08.
//!
//! The parts of day 08 use a grid of single-digit integers that represents the height of trees in
//! a forest. Each part focuses on "visibility" from a given point in the grid in different ways.

use anyhow::anyhow;

pub(crate) mod part_1;
pub(crate) mod part_2;

/// Indicates the direction of travel when checking visibility in the [TreeGrid].
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Represents a grid of trees with heights from 1-9.
struct TreeGrid {
    trees: Vec<Vec<u8>>,
}

impl TreeGrid {
    /// Create a TreeGrid from a string representation.
    ///
    /// The input grid should contain some number of rows with equal lengths. Each character in the
    /// input is treated as a single-digit number representing the height at that position in the
    /// grid.
    pub(crate) fn new(input: &str) -> Result<Self, anyhow::Error> {
        // check if there is input to process
        if input.is_empty() {
            return Err(anyhow!("input is empty"));
        }
        // build the rows and columns
        let mut trees = Vec::new();
        for line in input.split('\n') {
            let mut row = Vec::new();
            for c in line.chars() {
                // convert the char to a digit, returning an error if it cannot be converted
                match c.to_digit(10) {
                    Some(num) => row.push(num as u8),
                    None => return Err(anyhow!("could not parse token to u8: {}", c)),
                }
            }
            trees.push(row);
        }
        Ok(TreeGrid { trees })
    }

    /// Count the number of trees visible from outside the grid.
    ///
    /// A tree is considered visible from outside the grid if it can be seen from the top, bottom,
    /// left, or right. Trees on the edge are always considered visible. Visibility is blocked by
    /// trees of equal or greater height.
    pub(crate) fn count_visible(&self) -> u32 {
        let mut visible_count = 0;
        for y in 0..self.trees.len() {
            for x in 0..self.trees[0].len() {
                if self.check_visibility(x, y) {
                    visible_count += 1
                }
            }
        }
        visible_count
    }

    /// Find the max scenic score within the [TreeGrid].
    ///
    /// Computes the scenic score for every tree in the grid and returns the maximum. The scenic
    /// score is calculated by finding the number of trees that can be seen in each direction and
    /// multiplying them together. Trees can be seen if they are of equal or lesser height. Once
    /// a tree of equal or greater height is found no further trees are considered. Trees on the
    /// perimeter have a score of zero in the direction towards the edge.
    pub(crate) fn max_scenic_score(&self) -> u32 {
        let mut score = 0;
        // loop over each tree
        for y in 0..self.trees.len() {
            for x in 0..self.trees[0].len() {
                let tree_score = self.scenic_score(x, y);
                // keep only the highest scenic score
                if tree_score > score {
                    score = tree_score;
                }
            }
        }
        score
    }

    /// Check if a tree is visible from any side.
    fn check_visibility(&self, x: usize, y: usize) -> bool {
        let mut visible = false;
        if self.check_visibility_from_direction(x, y, Direction::Up) {
            visible = true;
        }
        if self.check_visibility_from_direction(x, y, Direction::Down) {
            visible = true;
        }
        if self.check_visibility_from_direction(x, y, Direction::Left) {
            visible = true;
        }
        if self.check_visibility_from_direction(x, y, Direction::Right) {
            visible = true;
        }
        visible
    }

    /// Check if a tree is visible from a particular direction.
    fn check_visibility_from_direction(&self, x: usize, y: usize, direction: Direction) -> bool {
        // trees on the perimeter are always considered visible
        if x == 0 || x == self.trees[x].len() - 1 || y == 0 || y == self.trees.len() - 1 {
            return true;
        }
        let tree_height = self.trees[y][x];
        let mut x = x;
        let mut y = y;
        let mut still_checking = true;
        // keep checking visibility until we have hit the edge
        while still_checking {
            // step coordinates based on direction
            (x, y) = self.coordinate_step(x, y, &direction);
            // check if tree is still visible
            if self.trees[y][x] >= tree_height {
                return false;
            }
            // break out of loop if we have hit the edge
            if self.at_edge(x, y) {
                still_checking = false;
            }
        }
        true
    }

    /// Calculate the scenic score of a tree.
    fn scenic_score(&self, x: usize, y: usize) -> u32 {
        let up = self.scenic_score_from_direction(x, y, Direction::Up);
        let down = self.scenic_score_from_direction(x, y, Direction::Down);
        let left = self.scenic_score_from_direction(x, y, Direction::Left);
        let right = self.scenic_score_from_direction(x, y, Direction::Right);
        up * down * left * right
    }

    /// Calculates the number of trees that can be seen in a particular direction.
    fn scenic_score_from_direction(&self, x: usize, y: usize, direction: Direction) -> u32 {
        // trees on the perimeter always have a score of zero
        if x == 0 || x == self.trees[x].len() - 1 || y == 0 || y == self.trees.len() - 1 {
            return 0;
        }
        let mut score = 0;
        let tree_height = self.trees[y][x];
        let mut x = x;
        let mut y = y;
        let mut still_checking = true;
        // keep checking visibility until we hit the edge or a tree of equal or greater height
        while still_checking {
            // increment score
            score += 1;
            // step coordinates based on direction
            (x, y) = self.coordinate_step(x, y, &direction);
            // stop if we hit a equal or taller tree or the edge
            if self.trees[y][x] >= tree_height || self.at_edge(x, y) {
                still_checking = false;
            }
        }
        score
    }

    /// Checks to see if the provided coordinates are on the perimeter of the [TreeGrid]
    fn at_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || x == self.trees[x].len() - 1 || y == 0 || y == self.trees.len() - 1
    }

    /// Steps coordinates by one based on the direction.
    fn coordinate_step(&self, x: usize, y: usize, direction: &Direction) -> (usize, usize) {
        let mut x = x;
        let mut y = y;
        match direction {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_visible() {
        let input = "30373
25512
65332
33549
35390";
        let tree_grid = TreeGrid::new(input).expect("should return result");
        assert_eq!(21, tree_grid.count_visible());
    }

    #[test]
    fn test_scenic_score() {
        let input = "30373
25512
65332
33549
35390";
        let tree_grid = TreeGrid::new(input).expect("should return result");
        assert_eq!(8, tree_grid.max_scenic_score());
    }
}
