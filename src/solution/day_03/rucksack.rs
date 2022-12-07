use anyhow::anyhow;
use std::collections::HashMap;

pub(super) struct Rucksack {
    first: HashMap<char, u32>,
    second: HashMap<char, u32>,
}

impl Rucksack {
    pub(super) fn new(input: &str) -> Result<Self, anyhow::Error> {
        if input.len() % 2 != 0 {
            return Err(anyhow!("input does not have even number of characters"));
        }
        let mut rucksack = Rucksack {
            first: HashMap::new(),
            second: HashMap::new(),
        };
        let (first, second) = input.split_at(input.len() / 2);
        for item in first.chars() {
            *rucksack.first.entry(item).or_insert(0) += 1;
        }
        for item in second.chars() {
            *rucksack.second.entry(item).or_insert(0) += 1;
        }
        Ok(rucksack)
    }

    pub(super) fn find_common(&self) -> Option<char> {
        for key in self.first.keys() {
            if self.second.contains_key(key) {
                return Some(*key);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::new(input).expect("should return result");
        let result = rucksack.find_common().expect("should have item in common");
        assert_eq!('p', result);
    }
}
