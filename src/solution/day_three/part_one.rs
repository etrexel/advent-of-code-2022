use crate::solution::day_three::{compute_priority, rucksack::Rucksack};

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let mut total = 0;
    for line in input.split('\n') {
        let rucksack = Rucksack::new(line)?;
        if let Some(val) = rucksack.find_common() {
            total += compute_priority(&val)?
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!("157", solve(input).expect("should return result"));
    }
}
