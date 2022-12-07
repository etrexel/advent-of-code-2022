use crate::solution::day_02::{score_round, Choice};
use anyhow::anyhow;

fn parse_choice(s: &str) -> Result<Choice, anyhow::Error> {
    match s {
        "A" | "X" => Ok(Choice::Rock),
        "B" | "Y" => Ok(Choice::Paper),
        "C" | "Z" => Ok(Choice::Scissors),
        _ => Err(anyhow!("couldn't convert input to Choice: {}", s)),
    }
}

fn parse_input(input: &str) -> Result<Vec<(Choice, Choice)>, anyhow::Error> {
    let mut output = Vec::<(Choice, Choice)>::new();
    for line in input.split('\n') {
        let components: Vec<&str> = line.split(' ').collect();
        if components.len() < 2 {
            return Err(anyhow!("invalid number of components on line: {}", &line));
        }
        output.push((parse_choice(components[0])?, parse_choice(components[1])?));
    }
    Ok(output)
}

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let games = parse_input(input)?;
    let mut total = 0;
    for game in games {
        total += score_round(&game.0, &game.1);
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "A X
B Y
C Z";
        let result = parse_input(input).expect("should return result");
        assert_eq!((Choice::Rock, Choice::Rock), result[0]);
        assert_eq!((Choice::Paper, Choice::Paper), result[1]);
        assert_eq!((Choice::Scissors, Choice::Scissors), result[2]);
    }

    #[test]
    fn test_solve() {
        let input = "A Y
B X
C Z";
        assert_eq!("15", solve(input).expect("should return string result"));
    }
}
