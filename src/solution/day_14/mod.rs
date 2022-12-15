// TODO: docs

use anyhow::anyhow;
use std::collections::HashSet;
use std::str::FromStr;

pub(crate) mod part_1;
pub(crate) mod part_2;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinates {
    x: u32,
    y: u32,
}

impl FromStr for Coordinates {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(anyhow!("invalid input for Coordinates: {}", s));
        }
        let x = parts[0].parse::<u32>()?;
        let y = parts[1].parse::<u32>()?;
        Ok(Self { x, y })
    }
}

struct Cave {
    walls: HashSet<Coordinates>,
    lowest_wall_point: u32,
    cave_floor: u32,
    sand_spawn: Coordinates,
    sand: HashSet<Coordinates>,
}

impl Cave {
    pub(crate) fn new(input: &str) -> Result<Self, anyhow::Error> {
        let mut walls = HashSet::new();
        for line in input.split('\n') {
            walls.extend(process_line(line)?);
        }
        let lowest_wall_point = walls.iter().map(|w| w.y).max().unwrap_or(0);
        let cave_floor = lowest_wall_point + 2;
        Ok(Self {
            walls,
            lowest_wall_point,
            cave_floor,
            sand_spawn: Coordinates { x: 500, y: 0 },
            sand: HashSet::new(),
        })
    }

    pub(crate) fn simulate_sand(&mut self, use_floor: bool) {
        let mut done = false;
        while !done {
            let mut new_sand = self.sand_spawn;
            let mut still_falling = true;
            while still_falling {
                if self.is_empty(new_sand.x, new_sand.y + 1) {
                    new_sand.y += 1;
                } else if self.is_empty(new_sand.x - 1, new_sand.y + 1) {
                    new_sand.x -= 1;
                    new_sand.y += 1;
                } else if self.is_empty(new_sand.x + 1, new_sand.y + 1) {
                    new_sand.x += 1;
                    new_sand.y += 1;
                } else {
                    self.sand.insert(new_sand);
                    still_falling = false;
                    if use_floor && new_sand == self.sand_spawn {
                        done = true;
                    }
                }
                if !use_floor && new_sand.y > self.lowest_wall_point {
                    still_falling = false;
                    done = true;
                }
            }
        }
    }

    pub(crate) fn count_sand(&self) -> u32 {
        self.sand.len() as u32
    }

    fn is_empty(&self, x: u32, y: u32) -> bool {
        let loc = Coordinates { x, y };
        !self.walls.contains(&loc) && !self.sand.contains(&loc) && self.cave_floor != y
    }
}

fn process_line(input: &str) -> Result<HashSet<Coordinates>, anyhow::Error> {
    let mut walls = HashSet::new();
    let coords = input
        .split("->")
        .into_iter()
        .map(|raw| raw.trim().parse::<Coordinates>())
        .collect::<Result<Vec<Coordinates>, anyhow::Error>>()?;
    for i in 1..coords.len() {
        walls.extend(build_wall(&coords[i - 1], &coords[i])?);
    }
    Ok(walls)
}

fn build_wall(
    start: &Coordinates,
    end: &Coordinates,
) -> Result<HashSet<Coordinates>, anyhow::Error> {
    let mut wall_segments = HashSet::new();
    if start.x != end.x && start.y != end.y {
        return Err(anyhow!(
            "wall start and end must share at least a row or column"
        ));
    }
    let same_x = start.x == end.x;
    let (shared_value, start_i, end_i) = match same_x {
        true => {
            let mut range = [start.y, end.y];
            range.sort();
            (start.x, range[0], range[1])
        }
        false => {
            let mut range = [start.x, end.x];
            range.sort();
            (start.y, range[0], range[1])
        }
    };
    for i in start_i..=end_i {
        match same_x {
            true => wall_segments.insert(Coordinates {
                x: shared_value,
                y: i,
            }),
            false => wall_segments.insert(Coordinates {
                x: i,
                y: shared_value,
            }),
        };
    }

    Ok(wall_segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cave_no_floor() {
        let input = "498,5 -> 502,5";
        let mut cave = Cave::new(input).expect("should return result");
        cave.simulate_sand(false);
        assert_eq!(4, cave.count_sand());
    }

    #[test]
    fn test_cave_with_floor() {
        let input = "498,5 -> 502,5";
        let mut cave = Cave::new(input).expect("should return result");
        cave.simulate_sand(true);
        assert_eq!(41, cave.count_sand());
    }
}
