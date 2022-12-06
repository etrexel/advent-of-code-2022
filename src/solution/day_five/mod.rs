mod parser;
pub(crate) mod part_one;
pub(crate) mod part_two;

struct StackGroup {
    stacks: Vec<Vec<char>>,
}

impl StackGroup {
    fn execute_command(&mut self, command: &Command, preserve_order: bool) {
        let mut buf = Vec::new();
        for _ in 0..command.count {
            if let Some(v) = self.stacks[command.source].pop() {
                match preserve_order {
                    true => buf.insert(0, v),
                    false => buf.push(v),
                }
            }
        }
        self.stacks[command.target].append(&mut buf);
    }

    fn top_of_stacks(&self) -> String {
        let mut output = String::new();
        for stack in &self.stacks {
            if let Some(c) = stack.last() {
                output.push(*c);
            }
        }
        output
    }
}

struct Command {
    source: usize,
    target: usize,
    count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_group() {
        let mut stack_group = StackGroup {
            stacks: vec![vec![], vec!['a', 'b', 'c'], vec![], vec!['x', 'y']],
        };
        let command = Command {
            source: 1,
            target: 2,
            count: 1,
        };
        stack_group.execute_command(&command, false);
        assert_eq!("bcy", stack_group.top_of_stacks());
    }

    #[test]
    fn test_execute_command_preserve_order() {
        let mut stack_group = StackGroup {
            stacks: vec![vec![], vec!['a', 'b', 'c'], vec!['f'], vec!['x', 'y']],
        };
        let command = Command {
            source: 1,
            target: 2,
            count: 2,
        };
        stack_group.execute_command(&command, true);
        assert_eq!("acy", stack_group.top_of_stacks());
    }
}
