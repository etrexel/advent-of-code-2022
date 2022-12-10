// TODO: docs

use anyhow::{anyhow, Context};

pub(crate) mod part_1;
pub(crate) mod part_2;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub(crate) fn new(input: &str) -> Result<Self, anyhow::Error> {
        if input.contains("noop") {
            return Ok(Instruction::Noop);
        } else if input.contains("addx") {
            let parts = input.split(' ').collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err(anyhow!("invalid input: {}", input));
            }
            let offset = parts[1]
                .parse::<i32>()
                .with_context(|| format!("could not parse token to i32: {}", parts[1]))?;
            return Ok(Instruction::Addx(offset));
        }
        Err(anyhow!("input is not an instruction"))
    }
}

struct Device {
    cycle: u32,
    register: i32,
    signal_strength: i32,
    display: Vec<Vec<char>>,
}

impl Device {
    pub(crate) fn new() -> Self {
        Device {
            cycle: 1,
            register: 1,
            signal_strength: 0,
            display: vec![vec!['.'; 40]; 6],
        }
    }

    pub(crate) fn execute_program(&mut self, instructions: &Vec<Instruction>, for_cycles: u32) {
        for instruction in instructions {
            self.execute_instruction(instruction);
            if self.cycle > for_cycles {
                break;
            }
        }
    }

    pub(crate) fn execute_instruction(&mut self, instruction: &Instruction) {
        let (cycles, register_update) = match instruction {
            Instruction::Noop => (1, 0),
            Instruction::Addx(count) => (2, *count),
        };
        for _ in 0..cycles {
            self.update_display();
            self.update_signal_strength();
            self.cycle += 1;
        }
        self.register += register_update;
    }

    pub(crate) fn get_signal_strength(&self) -> i32 {
        self.signal_strength
    }

    pub(crate) fn get_display(&self) -> String {
        let mut display = String::new();
        for row in &self.display {
            for c in row {
                display.push(*c);
            }
            display.push('\n');
        }
        display
    }

    fn update_signal_strength(&mut self) {
        if self.cycle > 19 && (self.cycle == 20 || (self.cycle - 20) % 40 == 0) {
            self.signal_strength += self.cycle as i32 * self.register
        }
    }

    fn update_display(&mut self) {
        let mut row = self.cycle / 40;
        if self.cycle % 40 == 0 {
            row -= 1;
        }
        let col = (self.cycle - 1) % 40;
        if (col as i32 - self.register).abs() <= 1 {
            self.display[row as usize][col as usize] = '#';
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, anyhow::Error> {
    let mut instructions = Vec::new();
    for line in input.split('\n') {
        instructions.push(Instruction::new(line)?);
    }
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_noop() {
        let input = "noop";
        assert_eq!(
            Instruction::Noop,
            Instruction::new(input).expect("should return result")
        );
    }

    #[test]
    fn test_instruction_addx() {
        let input = "addx 5";
        assert_eq!(
            Instruction::Addx(5),
            Instruction::new(input).expect("should return result")
        );
    }

    #[test]
    fn test_device() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let instructions = parse_input(input).expect("should return result");
        let mut device = Device::new();
        device.execute_program(&instructions, 220);
        assert_eq!(13140, device.get_signal_strength());
        let gt = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......######....................\n";
        assert_eq!(gt, format!("\n{}", device.get_display()));
    }
}
