//! Solvers for day 09.
//!
//! The parts of day 09 use a coordinate system to simulate a sort of rope physics. As the head of
//! the rope moves up, down, left, or right, the "tail" of the rope moves according to some rules.

use anyhow::{anyhow, Context};
use std::collections::HashMap;

pub(crate) mod part_1;
pub(crate) mod part_2;

/// Indicates the direction the head of the rope will travel.
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// A single movement of the rope head in a certain direction for a certain number of units.
struct Movement {
    direction: Direction,
    count: u32,
}

/// Represents whole-number coordinates within a grid.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    /// Creates a new set of coordinates at (0,0).
    pub(crate) fn new() -> Self {
        Coordinates { x: 0, y: 0 }
    }

    /// Update the coordinates based on the position of the another set of coordinates.
    ///
    /// This treats the input coordinates as this coordinate's "head" meaning that it will update
    /// based on the following criteria:
    /// - If the current coordinates are within one unit of the head it does not move
    /// - If the current coordinates are further than one in either direction they move
    ///
    /// When a coordinate moves it moves one unit toward the head. If either axis is equal the
    /// coordinates move one unit on that axis towards the head. If both axes are different the
    /// coordinates move diagonally towards the head.
    pub(crate) fn update(&mut self, other: &Coordinates) {
        // if we are close enough we don't have to move
        if self.within_one(other) {
            return;
        }
        // update x if we are not on the same row
        if self.x != other.x {
            match self.x < other.x {
                true => self.x += 1,
                false => self.x -= 1,
            }
        }
        // update y if we are not on the same column
        if self.y != other.y {
            match self.y < other.y {
                true => self.y += 1,
                false => self.y -= 1,
            }
        }
    }

    /// Checks that the current coordinates are at most one unit away on both axes.
    ///
    /// A coordinate that is one unit up and one unit over is still considered to be within one.
    fn within_one(&self, other: &Coordinates) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

/// Represents a rope with "knots" being moved around in a coordinate system.
///
/// A rope can have some number of knots after the head greater than zero. When the head moves
/// according to a [Movement] each knot follows the knot in front of it according to the rules
/// defined in [Coordinates::update]. Each unique set of coordinates visited by the last knot in the rope is
/// tracked.
struct Rope {
    knots: Vec<Coordinates>,
    visited: HashMap<Coordinates, bool>,
}

impl Rope {
    /// Creates a new rope with the provided number of knots.
    pub(crate) fn new(knot_count: u32) -> Result<Self, anyhow::Error> {
        // should have at least one knot other than the head
        if knot_count == 0 {
            return Err(anyhow!("must have at least one knot"));
        }
        let mut visited = HashMap::new();
        let mut knots = Vec::new();
        knots.push(Coordinates::new());
        for _ in 0..knot_count {
            knots.push(Coordinates::new());
        }
        // the starting square is always visited
        visited.insert(Coordinates::new(), true);
        Ok(Rope { knots, visited })
    }

    /// Convenience method for processing a list of [Movement] objects.
    pub(crate) fn process_moves(&mut self, movements: &Vec<Movement>) {
        for movement in movements {
            self.process_move(movement);
        }
    }

    /// Update the coordinates of each knot based on a [Movement].
    ///
    /// The head of the rope moves in some direction some number of times and each time it moves
    /// all knots are updated following the knot in front of it. When a new coordinate is visited
    /// in the grid it is added to the list of visited coordinates.
    pub(crate) fn process_move(&mut self, movement: &Movement) {
        // update the knots n number of times based on the movement
        for _ in 0..movement.count {
            // move the head
            match movement.direction {
                Direction::Up => self.knots[0].y += 1,
                Direction::Down => self.knots[0].y -= 1,
                Direction::Left => self.knots[0].x -= 1,
                Direction::Right => self.knots[0].x += 1,
            }
            // update all subsequent knots based on the knot in front of it
            for i in 1..self.knots.len() {
                let prev = self.knots[i - 1];
                self.knots[i].update(&prev);
            }
            // insert will add if not exists, or overwrite if exists so we don't need to check
            self.visited.insert(self.knots[self.knots.len() - 1], true);
        }
    }

    /// Return the number of unique coordinates visited by the last knot in the rope.
    pub(crate) fn tail_visit_count(&self) -> u32 {
        // We only track visited nodes, so just return the count
        self.visited.values().len() as u32
    }
}

/// Parse a string input into a list of [Movement] objects.
///
/// Each line in the input should be of the form `<Direction> <Count>`, where:
/// - Direction is one of: `U`, `D`, `L`, `R`, representing up, down, left, and right
/// - Count is a natural number
fn parse_input(input: &str) -> Result<Vec<Movement>, anyhow::Error> {
    let mut movements = Vec::new();
    // loop over each line in the input
    for line in input.split('\n') {
        let parts = line.split(' ').collect::<Vec<&str>>();
        // each line should have exactly two parts: the direction token and the count token
        if parts.len() != 2 {
            return Err(anyhow!("invalid line: {}", line));
        }
        // get the direction
        let direction = match parts[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(anyhow!("invalid direction: {}", parts[0])),
        };
        // get the count
        let count = parts[1]
            .parse::<u32>()
            .with_context(|| format!("could not convert token to u32: {}", parts[1]))?;
        movements.push(Movement { direction, count });
    }
    Ok(movements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_no_move() {
        let head = Coordinates { x: 1, y: 1 };
        let mut tail = Coordinates { x: 0, y: 0 };
        tail.update(&head);
        assert_eq!(Coordinates { x: 0, y: 0 }, tail);
    }

    #[test]
    fn test_coordinate_two_over_one_up() {
        let head = Coordinates { x: 2, y: 1 };
        let mut tail = Coordinates { x: 0, y: 0 };
        tail.update(&head);
        assert_eq!(Coordinates { x: 1, y: 1 }, tail);
    }

    #[test]
    fn test_coordinate_one_over_two_up() {
        let head = Coordinates { x: 1, y: 2 };
        let mut tail = Coordinates { x: 0, y: 0 };
        tail.update(&head);
        assert_eq!(Coordinates { x: 1, y: 1 }, tail);
    }

    #[test]
    fn test_coordinate_diagonals() {
        let head = Coordinates { x: 2, y: 2 };
        let mut tail = Coordinates { x: 0, y: 0 };
        tail.update(&head);
        assert_eq!(Coordinates { x: 1, y: 1 }, tail);
        let head = Coordinates { x: -2, y: 2 };
        let mut tail = Coordinates { x: 0, y: 0 };
        tail.update(&head);
        assert_eq!(Coordinates { x: -1, y: 1 }, tail);
        let head = Coordinates { x: 2, y: -2 };
        let mut tail = Coordinates { x: 0, y: 0 };
        tail.update(&head);
        assert_eq!(Coordinates { x: 1, y: -1 }, tail);
        let head = Coordinates { x: -2, y: -2 };
        let mut tail = Coordinates { x: 0, y: 0 };
        tail.update(&head);
        assert_eq!(Coordinates { x: -1, y: -1 }, tail);
    }

    #[test]
    fn test_rope() {
        let movements = vec![
            Movement {
                direction: Direction::Right,
                count: 4,
            },
            Movement {
                direction: Direction::Up,
                count: 4,
            },
            Movement {
                direction: Direction::Left,
                count: 3,
            },
            Movement {
                direction: Direction::Down,
                count: 1,
            },
            Movement {
                direction: Direction::Right,
                count: 4,
            },
            Movement {
                direction: Direction::Down,
                count: 1,
            },
            Movement {
                direction: Direction::Left,
                count: 5,
            },
            Movement {
                direction: Direction::Right,
                count: 2,
            },
        ];
        let mut rope = Rope::new(1).expect("should return result");
        rope.process_moves(&movements);
        assert_eq!(13, rope.tail_visit_count());
    }

    #[test]
    fn test_parse_input() {
        let input = "R 4
L 2
D 1
U 5";
        let movements = parse_input(input).expect("should return result");
        assert_eq!(Direction::Right, movements[0].direction);
        assert_eq!(4, movements[0].count);
        assert_eq!(Direction::Down, movements[2].direction);
        assert_eq!(1, movements[2].count);
    }
}
