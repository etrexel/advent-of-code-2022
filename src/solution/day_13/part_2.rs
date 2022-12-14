// TODO: docs

use crate::solution::day_13::Packet;
use anyhow::anyhow;

fn parse_input(input: &str) -> Result<Vec<Packet>, anyhow::Error> {
    let mut packets = Vec::new();
    for p in input.replace("\n\n", "\n").split('\n') {
        packets.push(Packet::new(p)?);
    }
    Ok(packets)
}

fn marker_packet(marker: i32) -> Packet {
    Packet::Array(vec![Packet::Int(marker)])
}

fn find_marker_packet(packets: &[Packet], marker: i32) -> Result<usize, anyhow::Error> {
    packets
        .iter()
        .position(|p| *p == marker_packet(marker))
        .ok_or_else(|| anyhow!("could not find marker packet {}", marker))
}

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let mut packets = parse_input(input)?;
    packets.push(marker_packet(2));
    packets.push(marker_packet(6));
    packets.sort();
    let marker_2_idx = find_marker_packet(&packets, 2)?;
    let marker_6_idx = find_marker_packet(&packets, 6)?;
    let key = (marker_2_idx + 1) as u32 * (marker_6_idx + 1) as u32;
    Ok(key.to_string())
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
        assert_eq!("140", solve(input).expect("should return result"));
    }
}
