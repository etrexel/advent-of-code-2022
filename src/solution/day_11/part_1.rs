// TODO: docs

use crate::solution::day_11::Monkeys;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let mut monkeys = Monkeys::new(input, Some(3))?;
    monkeys.execute_rounds(20)?;
    Ok(monkeys.calculate_monkey_business().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!("10605", solve(input).expect("should return result"));
    }
}
