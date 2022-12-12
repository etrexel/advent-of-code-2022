// TODO: docs

use anyhow::anyhow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub(crate) mod part_1;
pub(crate) mod part_2;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Coordinates { x, y }
    }
}

struct Map {
    terrain: HashMap<Coordinates, u32>,
    start: Coordinates,
    end: Coordinates,
}

impl Map {
    pub(crate) fn new(input: &str) -> Result<Self, anyhow::Error> {
        let mut terrain = HashMap::new();
        let mut start = Coordinates { x: 0, y: 0 };
        let mut end = Coordinates { x: 0, y: 0 };
        for (y, line) in input.split('\n').enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start.x = x;
                    start.y = y;
                } else if c == 'E' {
                    end.x = x;
                    end.y = y;
                }
                terrain.insert(Coordinates::new(x, y), map_char_to_elevation(c)?);
            }
        }
        Ok(Self {
            terrain,
            start,
            end,
        })
    }

    pub(crate) fn shortest_path_length(&self) -> Result<u32, anyhow::Error> {
        let distances = self.dijkstra(true)?;
        let distance_from_start = distances
            .get(&self.start)
            .ok_or_else(|| anyhow!("distances should have every node"))?;
        Ok(*distance_from_start)
    }

    pub(crate) fn shortest_path_length_from_base(&self) -> Result<u32, anyhow::Error> {
        let distances = self.dijkstra(false)?;
        let mut min = u32::MAX;
        for loc in self.terrain.iter().filter(|c| *c.1 == 0) {
            let distance = distances
                .get(loc.0)
                .ok_or_else(|| anyhow!("distances should have every node"))?;
            if *distance < min {
                min = *distance
            }
        }
        Ok(min)
    }

    fn dijkstra(&self, exit_early: bool) -> Result<HashMap<Coordinates, u32>, anyhow::Error> {
        let mut unvisited = HashSet::new();
        let mut distance = HashMap::new();
        for coord in self.terrain.keys() {
            distance.insert(*coord, u32::MAX);
            unvisited.insert(*coord);
        }
        distance.insert(self.end, 0);

        while !unvisited.is_empty() {
            // could be optimized by keeping track of min on the fly
            let current = get_min_distance(&unvisited, &distance)?;
            if current == self.start && exit_early {
                return Ok(distance);
            }
            unvisited.remove(&current);
            if *distance
                .get(&current)
                .ok_or_else(|| anyhow!("distance should have every node"))?
                == u32::MAX
            {
                continue;
            }

            let mut targets = Vec::new();
            targets.push(Coordinates::new(current.x + 1, current.y));
            targets.push(Coordinates::new(current.x, current.y + 1));
            if current.x > 0 {
                targets.push(Coordinates::new(current.x - 1, current.y));
            }
            if current.y > 0 {
                targets.push(Coordinates::new(current.x, current.y - 1));
            }
            for target in targets {
                if self.can_pass(&current, &target)? && unvisited.contains(&target) {
                    let new_dist = distance
                        .get(&current)
                        .ok_or_else(|| anyhow!("distance should have every node"))?
                        + 1;
                    if new_dist
                        < *distance
                            .get(&target)
                            .ok_or_else(|| anyhow!("distance should have every node"))?
                    {
                        distance.insert(target, new_dist);
                    }
                }
            }
        }

        Ok(distance)
    }

    fn can_pass(&self, current: &Coordinates, target: &Coordinates) -> Result<bool, anyhow::Error> {
        if !self.terrain.contains_key(target) {
            return Ok(false);
        }
        let current_height = self
            .terrain
            .get(current)
            .ok_or_else(|| anyhow!("terrain should have every node"))?;
        let target_height = self
            .terrain
            .get(target)
            .ok_or_else(|| anyhow!("terrain should have every node"))?;
        // working backwards from end to start we can go down only one but up an unlimited amount
        Ok(*current_height as i32 - *target_height as i32 <= 1)
    }
}

fn map_char_to_elevation(c: char) -> Result<u32, anyhow::Error> {
    if c != 'S' && c != 'E' && !c.is_ascii_lowercase() {
        return Err(anyhow!("invalid input char: {}", c));
    }
    match c {
        'S' => Ok(0),
        'E' => Ok(25),
        _ => Ok((c as u32) - 97),
    }
}

fn get_min_distance(
    unvisited: &HashSet<Coordinates>,
    distance: &HashMap<Coordinates, u32>,
) -> Result<Coordinates, anyhow::Error> {
    let mut coords = Coordinates::new(0, 0);
    let mut min = u32::MAX;
    for c in unvisited.iter() {
        let distance = distance
            .get(c)
            .ok_or_else(|| anyhow!("distance should contain coordinates"))?;
        if *distance <= min {
            coords = *c;
            min = *distance;
        }
    }
    Ok(coords)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let map = Map::new(input).expect("should return result");
        assert_eq!(
            31,
            map.shortest_path_length().expect("should return result")
        );
    }
}
