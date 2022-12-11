use anyhow::{anyhow, Context};

pub(crate) mod part_1;
pub(crate) mod part_2;

enum Operator {
    Add,
    Multiply,
}

enum Other {
    Old,
    Value(u64),
}

struct Operation {
    operator: Operator,
    other: Other,
}

impl Operation {
    pub(crate) fn new(input: &str) -> Result<Self, anyhow::Error> {
        let parts = input.split(' ').collect::<Vec<&str>>();
        if parts.len() != 6 {
            return Err(anyhow!("invalid input for operation: {}", input));
        }
        let operator = match parts[4] {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => return Err(anyhow!("invalid value for operator: {}", parts[4])),
        };
        let other = match parts[5] {
            "old" => Other::Old,
            _ => {
                let value = parts[5]
                    .parse::<u64>()
                    .with_context(|| format!("could not parse token to u64: {}", parts[5]))?;
                Other::Value(value)
            }
        };
        Ok(Self { operator, other })
    }

    pub(crate) fn perform(&self, item: u64) -> u64 {
        let value = match self.other {
            Other::Old => item,
            Other::Value(val) => val,
        };
        match self.operator {
            Operator::Add => item + value,
            Operator::Multiply => item * value,
        }
    }
}

struct Decision {
    value: u64,
    true_target: usize,
    false_target: usize,
}

impl Decision {
    pub(crate) fn new(input: &str) -> Result<Self, anyhow::Error> {
        let parts = input.split('\n').collect::<Vec<&str>>();
        if parts.len() != 3 || !parts[1].contains("true") || !parts[2].contains("false") {
            return Err(anyhow!("invalid input for decision"));
        }
        let value_str = parts[0]
            .split(' ')
            .last()
            .ok_or_else(|| anyhow!("could not find decision value in line: {}", parts[0]))?;
        let value = value_str
            .parse::<u64>()
            .with_context(|| format!("could not parse token into u64: {}", value_str))?;
        let true_target_str = parts[1]
            .split(' ')
            .last()
            .ok_or_else(|| anyhow!("could not find decision true target in line: {}", parts[1]))?;
        let true_target = true_target_str
            .parse::<usize>()
            .with_context(|| format!("could not parse token into usize: {}", true_target_str))?;
        let false_target_str = parts[2]
            .split(' ')
            .last()
            .ok_or_else(|| anyhow!("could not find decision false target in line: {}", parts[2]))?;
        let false_target = false_target_str
            .parse::<usize>()
            .with_context(|| format!("could not parse token into usize: {}", false_target_str))?;
        Ok(Self {
            value,
            true_target,
            false_target,
        })
    }

    pub(crate) fn decide(&self, item: u64) -> usize {
        match item % self.value == 0 {
            true => self.true_target,
            false => self.false_target,
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    decision: Decision,
    inspections: u64,
    reduce_worry: u64,
}

impl Monkey {
    pub(crate) fn new(input: &str, reduce_worry: Option<u64>) -> Result<Self, anyhow::Error> {
        let mut parts = input.split('\n').collect::<Vec<&str>>();
        if parts.len() != 6 {
            return Err(anyhow!("invalid number of lines for monkey"));
        }
        // strip prefixes
        for (i, part) in parts.iter_mut().enumerate().skip(1) {
            *part = part
                .strip_prefix("  ")
                .ok_or_else(|| anyhow!("invalid indentation in line: {}", i + 1))?;
        }
        // construct starting items
        let mut items = Vec::new();
        for item in parts[1].split(' ').skip(2) {
            items.push(
                item.trim_matches(',')
                    .parse::<u64>()
                    .with_context(|| format!("could not parse token to u64: {}", item))?,
            )
        }
        // construct operation
        let operation = Operation::new(parts[2])?;
        // construct decision
        let decision_input = [parts[3], "\n", parts[4], "\n", parts[5]].concat();
        let decision = Decision::new(&decision_input)?;
        let reduce_worry = reduce_worry.unwrap_or(1);
        Ok(Self {
            items,
            operation,
            decision,
            inspections: 0,
            reduce_worry,
        })
    }

    pub(crate) fn inspect_items(&mut self) {
        for i in 0..self.items.len() {
            self.inspections += 1;
            self.items[i] = self.operation.perform(self.items[i]);
            self.items[i] /= self.reduce_worry;
        }
    }

    pub(crate) fn throw_items(&mut self) -> Vec<(usize, u64)> {
        let mut thrown_items = Vec::new();
        if self.items.is_empty() {
            return thrown_items;
        }
        for item in &self.items {
            thrown_items.push((self.decision.decide(*item), *item));
        }
        self.items.clear();
        thrown_items
    }

    pub(crate) fn catch_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

struct Monkeys {
    monkeys: Vec<Monkey>,
    field_bound: u64,
}

impl Monkeys {
    pub(crate) fn new(input: &str, reduce_worry: Option<u64>) -> Result<Self, anyhow::Error> {
        let mut monkeys = Vec::new();
        for monkey_input in input.split("\n\n") {
            monkeys.push(Monkey::new(monkey_input, reduce_worry)?);
        }
        let field_bound = monkeys.iter().map(|m| m.decision.value).product::<u64>();
        Ok(Self {
            monkeys,
            field_bound,
        })
    }

    pub(crate) fn execute_rounds(&mut self, rounds: u32) -> Result<(), anyhow::Error> {
        for _ in 0..rounds {
            for i in 0..self.monkeys.len() {
                self.monkeys[i].inspect_items();
                for (target, item) in self.monkeys[i].throw_items() {
                    if target >= self.monkeys.len() {
                        return Err(anyhow!("tried to throw item to non-existent monkey"));
                    }
                    self.monkeys[target].catch_item(item % self.field_bound);
                }
            }
        }
        Ok(())
    }

    pub(crate) fn calculate_monkey_business(&self) -> u64 {
        let mut inspections = self
            .monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<Vec<u64>>();
        inspections.sort();
        inspections.reverse();
        match inspections.len() {
            0 => 0,
            1 => inspections[0],
            _ => inspections[0] * inspections[1],
        }
    }
}
