use anyhow::{anyhow, Context};

pub(crate) mod part_one;
pub(crate) mod part_two;

#[derive(Debug, PartialEq, Eq)]
struct Assignment {
    upper: u32,
    lower: u32,
}

impl Assignment {
    fn new(lower: u32, upper: u32) -> Self {
        Self { upper, lower }
    }

    fn contains(&self, other: &Assignment) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        (other.lower >= self.lower && other.lower <= self.upper)
            || (other.upper >= self.lower && other.upper <= self.upper)
            || (self.lower >= other.lower && self.lower <= other.upper)
            || (self.upper >= other.lower && self.upper <= other.upper)
    }
}

fn parse_input(input: &str) -> Result<Vec<(Assignment, Assignment)>, anyhow::Error> {
    let mut pairs = Vec::new();
    for line in input.split("\n") {
        pairs.push(build_assignment_pair(line)?);
    }
    Ok(pairs)
}

fn build_assignment_pair(input: &str) -> Result<(Assignment, Assignment), anyhow::Error> {
    let parts: Vec<&str> = input.split(",").collect();
    if parts.len() != 2 {
        return Err(anyhow!("invalid assignment pair: {}", input));
    }
    Ok((build_assignment(parts[0])?, build_assignment(parts[1])?))
}

fn build_assignment(input: &str) -> Result<Assignment, anyhow::Error> {
    let parts: Vec<&str> = input.split("-").collect();
    if parts.len() != 2 {
        return Err(anyhow!("invalid assignment string: {}", input));
    }
    let lower = parts[0].parse::<u32>().context("lk")?;
    let upper = parts[1].parse::<u32>().context("lk")?;
    Ok(Assignment::new(lower, upper))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let assignments = parse_input(input).expect("should return result");
        assert_eq!(Assignment { lower: 2, upper: 4 }, assignments[0].0);
        assert_eq!(Assignment { lower: 6, upper: 8 }, assignments[0].1);
        assert_eq!(Assignment { lower: 6, upper: 6 }, assignments[4].0);
        assert_eq!(Assignment { lower: 4, upper: 8 }, assignments[5].1);
    }

    #[test]
    fn test_assignment_contains() {
        let outer = Assignment { lower: 2, upper: 7 };
        let inner = Assignment { lower: 3, upper: 5 };
        assert_eq!(true, outer.contains(&inner));
    }

    #[test]
    fn test_assignment_overlap() {
        let left = Assignment { lower: 2, upper: 5 };
        let right = Assignment { lower: 4, upper: 7 };
        assert_eq!(true, left.overlaps(&right));
    }
}
