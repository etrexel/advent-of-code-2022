// TODO: docs

use crate::solution::day_13::Packet;
use anyhow::anyhow;

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    pub(crate) fn new(input: &str) -> Result<Self, anyhow::Error> {
        let parts = input.split('\n').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(anyhow!("invalid number of parts for PacketPair"));
        }
        Ok(PacketPair {
            left: Packet::new(parts[0])?,
            right: Packet::new(parts[1])?,
        })
    }

    pub(crate) fn in_order(&self) -> bool {
        self.left <= self.right
    }
}

fn parse_input(input: &str) -> Result<Vec<PacketPair>, anyhow::Error> {
    let mut pairs = Vec::new();
    for p in input.split("\n\n") {
        pairs.push(PacketPair::new(p)?);
    }
    Ok(pairs)
}

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let pairs = parse_input(input)?;
    let mut in_order_idx = Vec::new();
    for (idx, pair) in pairs.iter().enumerate() {
        if pair.in_order() {
            in_order_idx.push((idx + 1) as u32);
        }
    }
    Ok(in_order_idx.iter().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!("13", solve(input).expect("should return result"));
    }
}
