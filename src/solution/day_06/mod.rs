use std::collections::{HashMap, VecDeque};

pub(crate) mod part_1;
pub(crate) mod part_2;

fn decode_bitstream(input: &str, unique_count: usize) -> u32 {
    let mut result = 0;
    let mut buf: VecDeque<char> = VecDeque::with_capacity(unique_count);
    for (i, elem) in input.chars().enumerate() {
        buf.push_back(elem);
        if buf.len() < unique_count {
            continue;
        }
        if contains_all_unique(&buf) {
            result = i + 1;
            break;
        }
        buf.pop_front();
    }
    result as u32
}

fn contains_all_unique(queue: &VecDeque<char>) -> bool {
    let mut count = HashMap::new();
    for elem in queue {
        match count.contains_key(elem) {
            true => return false,
            false => {
                count.insert(elem, true);
            }
        }
    }
    true
}
