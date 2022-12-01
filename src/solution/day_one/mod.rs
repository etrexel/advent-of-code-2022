use anyhow::Context;

pub(crate) mod part_one;
pub(crate) mod part_two;

/// Sum individual groups separated by newlines
fn parse_input(input: &str) -> Result<Vec<u32>, anyhow::Error> {
    let mut output = Vec::<u32>::new();
    output.push(0);
    let mut idx = 0;
    for line in input.split("\n") {
        if line == "" {
            output.push(0);
            idx += 1;
            continue;
        }
        output[idx] += line.parse::<u32>().context("could not parse line to u32")?;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(
            vec!(6000, 4000, 11000, 24000, 10000),
            parse_input(&input).expect("should return vec")
        )
    }
}
