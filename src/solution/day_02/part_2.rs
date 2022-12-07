use crate::solution::day_02::{score_round, Choice};
use anyhow::anyhow;

fn parse_choice(s: &str) -> Result<Choice, anyhow::Error> {
    match s {
        "A" => Ok(Choice::Rock),
        "B" => Ok(Choice::Paper),
        "C" => Ok(Choice::Scissors),
        _ => Err(anyhow!("couldn't convert input to Choice: {}", s)),
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

fn parse_outcome(s: &str) -> Result<Outcome, anyhow::Error> {
    match s {
        "X" => Ok(Outcome::Loss),
        "Y" => Ok(Outcome::Draw),
        "Z" => Ok(Outcome::Win),
        _ => Err(anyhow!("couldn't convert input to Outcome: {}", s)),
    }
}

fn parse_input(input: &str) -> Result<Vec<(Choice, Outcome)>, anyhow::Error> {
    let mut output = Vec::<(Choice, Outcome)>::new();
    for line in input.split('\n') {
        let components: Vec<&str> = line.split(' ').collect();
        if components.len() < 2 {
            return Err(anyhow!("invalid number of components on line: {}", &line));
        }
        output.push((parse_choice(components[0])?, parse_outcome(components[1])?));
    }
    Ok(output)
}

fn build_game(opponent: &Choice, outcome: &Outcome) -> (Choice, Choice) {
    let player = match opponent {
        Choice::Rock => match outcome {
            Outcome::Win => Choice::Paper,
            Outcome::Draw => Choice::Rock,
            Outcome::Loss => Choice::Scissors,
        },
        Choice::Paper => match outcome {
            Outcome::Win => Choice::Scissors,
            Outcome::Draw => Choice::Paper,
            Outcome::Loss => Choice::Rock,
        },
        Choice::Scissors => match outcome {
            Outcome::Win => Choice::Rock,
            Outcome::Draw => Choice::Scissors,
            Outcome::Loss => Choice::Paper,
        },
    };
    (*opponent, player)
}

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let codes = parse_input(input)?;
    let mut games = Vec::<(Choice, Choice)>::new();
    for code in codes {
        games.push(build_game(&code.0, &code.1))
    }
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
        assert_eq!((Choice::Rock, Outcome::Loss), result[0]);
        assert_eq!((Choice::Paper, Outcome::Draw), result[1]);
        assert_eq!((Choice::Scissors, Outcome::Win), result[2]);
    }

    #[test]
    fn test_solve() {
        let input = "A Y
B X
C Z";
        assert_eq!("12", solve(input).expect("should return string result"));
    }
}
