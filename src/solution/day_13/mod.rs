// TODO: docs

use anyhow::{anyhow, Context};
use std::cmp::Ordering;

pub(crate) mod part_1;
pub(crate) mod part_2;

#[derive(PartialEq, Eq, Debug)]
enum Packet {
    Int(i32),
    Array(Vec<Packet>),
}

impl Packet {
    pub(crate) fn new(input: &str) -> Result<Self, anyhow::Error> {
        if input.len() < 2 || !input.starts_with('[') || !input.ends_with(']') {
            return Err(anyhow!("invalid packet input"));
        }
        let trimmed_input = trim_first_and_last(input);
        let mut packets = Vec::new();
        let mut scratch = String::new();
        let mut arr_stack = 0;
        for c in trimmed_input.chars() {
            if c == ' ' {
                continue;
            }
            if c == '[' {
                scratch.push(c);
                arr_stack += 1;
            } else if c == ']' {
                arr_stack -= 1;
                scratch.push(c);
            } else if c == ',' {
                if arr_stack > 0 {
                    scratch.push(c);
                } else {
                    packets.push(process_scratch(&scratch)?);
                    scratch.clear();
                }
            } else {
                scratch.push(c);
            }
        }
        if !scratch.is_empty() {
            packets.push(process_scratch(&scratch)?);
        }
        Ok(Packet::Array(packets))
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match &self {
            Packet::Int(sv) => match &other {
                // both are ints
                Packet::Int(ov) => sv.cmp(ov),
                // self is an int and other is an array
                Packet::Array(_) => Packet::Array(vec![Packet::Int(*sv)]).cmp(other),
            },
            Packet::Array(sa) => match &other {
                // self is an array and other is an int
                Packet::Int(ov) => self.cmp(&Packet::Array(vec![Packet::Int(*ov)])),
                // both are arrays
                Packet::Array(oa) => {
                    // compare all the elements we have
                    let compare_length = match sa.len() > oa.len() {
                        true => oa.len(),
                        false => sa.len(),
                    };
                    for i in 0..compare_length {
                        // return first non-equal result (this was not intuitive based on description)
                        let compare = sa[i].cmp(&oa[i]);
                        if compare.is_ne() {
                            return compare;
                        }
                    }
                    // if all elements are equal we check the length
                    sa.len().cmp(&oa.len())
                }
            },
        }
    }
}

fn trim_first_and_last(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn process_scratch(input: &str) -> Result<Packet, anyhow::Error> {
    if input.is_empty() {
        return Err(anyhow!("scratch is empty"));
    }
    match input.contains('[') {
        true => Packet::new(input),
        false => {
            let val = input
                .parse::<i32>()
                .with_context(|| anyhow!("could not parse scratch to i32: {}", input))?;
            Ok(Packet::Int(val))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_comparison() {
        let left_input = "[[[8,10,5],[9,1,[]],2,4,6],[5],[[0,9,2],[],1,3,[8,6]]]";
        let left = Packet::new(left_input).expect("should return result");
        let right_input = "[[],[[[6,7,0,5],[3],[10,7,7],7],[[2,4,0,1],[7,1],[7,10,4,4,4]],[[10,4,10],[],[4,6,8],[6,0],[1,7,8,7,9]]]]";
        let right = Packet::new(right_input).expect("should return result");
        assert!(left > right);
    }
}
