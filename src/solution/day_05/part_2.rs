// TODO: docs

use crate::solution::day_05::parser::parse_input;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let (mut stack_group, commands) = parse_input(input)?;
    for command in commands {
        stack_group.execute_command(&command, true);
    }
    Ok(stack_group.top_of_stacks())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!("MCD", solve(input).expect("should return result"));
    }
}
