use crate::solution::day_05::{Command, StackGroup};
use anyhow::{anyhow, Context};

pub(super) fn parse_input(input: &str) -> Result<(StackGroup, Vec<Command>), anyhow::Error> {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Err(anyhow!("invalid input"));
    }
    let stack_group = parse_stack_group(parts[0])?;
    let commands = parse_commands(parts[1])?;
    Ok((stack_group, commands))
}

pub(super) fn parse_stack_group(input: &str) -> Result<StackGroup, anyhow::Error> {
    let mut parts = input.split('\n').collect::<Vec<&str>>();
    parts.reverse();
    if parts.len() < 2 {
        return Err(anyhow!("invalid input"));
    }
    // resize all lines to be at least as long as required
    let mut raw_stack_frames = Vec::new();
    let line_length = parts[0].len();
    for i in 1..parts.len() {
        let new_frame = if parts[i].len() < line_length {
            let extend_size = parts[0].len() - parts[i].len();
            let extend = (0..extend_size).map(|_| " ").collect::<String>();
            [parts[i], &extend].concat()
        } else {
            parts[i].to_string()
        };
        raw_stack_frames.push(new_frame);
    }
    // populate the stacks
    let stack_count = match parts[0].split(' ').last() {
        Some(count) => count
            .parse::<usize>()
            .with_context(|| format!("could not parse token to usize: {}", count))?,
        None => return Err(anyhow!("no stacks present")),
    };
    let mut stacks: Vec<Vec<char>> = vec![Vec::new()];
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }
    for (i, stack) in stacks.iter_mut().enumerate().skip(1) {
        let item_idx = match parts[0].find(&i.to_string()) {
            Some(idx) => idx,
            None => return Err(anyhow!("could not find index of stack: {}", i)),
        };
        for raw_stack_frame in &raw_stack_frames {
            if let Some(item) = raw_stack_frame.chars().nth(item_idx) {
                if item.ne(&' ') {
                    stack.push(item);
                }
            }
        }
    }
    Ok(StackGroup { stacks })
}

pub(super) fn parse_commands(input: &str) -> Result<Vec<Command>, anyhow::Error> {
    let mut commands = Vec::new();
    for line in input.split('\n') {
        commands.push(parse_command(line)?)
    }
    Ok(commands)
}

pub(super) fn parse_command(input: &str) -> Result<Command, anyhow::Error> {
    let parts = input.split(' ').collect::<Vec<&str>>();
    if parts.len() != 6 {
        return Err(anyhow!("invalid input string: {}", input));
    }
    Ok(Command {
        source: parts[3]
            .parse::<usize>()
            .with_context(|| format!("could not convert token to usize: {}", parts[3]))?,
        target: parts[5]
            .parse::<usize>()
            .with_context(|| format!("could not convert token to usize: {}", parts[5]))?,
        count: parts[1]
            .parse::<u32>()
            .with_context(|| format!("could not convert token to u32: {}", parts[1]))?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let input = "move 3 from 2 to 1";
        let result = parse_command(input).expect("should return result");
        assert_eq!(2, result.source);
        assert_eq!(1, result.target);
        assert_eq!(3, result.count);
    }

    #[test]
    fn test_parse_commands() {
        let input = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = parse_commands(input).expect("should return result");
        assert_eq!(3, result[1].count);
        assert_eq!(2, result[2].source);
        assert_eq!(2, result[3].target);
    }

    #[test]
    fn test_parse_stack_group() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3";
        let stack_group = parse_stack_group(input).expect("should return result");
        assert_eq!('N', stack_group.stacks[1][1]);
        assert_eq!('C', stack_group.stacks[2][1]);
        assert_eq!('P', stack_group.stacks[3][0]);
    }

    #[test]
    fn test_parse_input() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let (stack_group, commands) = parse_input(input).expect("should return result");
        assert_eq!('C', stack_group.stacks[2][1]);
        assert_eq!(2, commands[2].source);
    }
}
