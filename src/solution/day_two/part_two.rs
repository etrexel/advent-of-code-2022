use crate::solution::day_two::{score_round, Choice};
use anyhow::anyhow;

fn parse_choice(s: &str) -> Result<Choice, anyhow::Error> {
    match s {
        "A" => Ok(Choice::ROCK),
        "B" => Ok(Choice::PAPER),
        "C" => Ok(Choice::SCISSORS),
        _ => Err(anyhow!("couldn't convert input to Choice: {}", s)),
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Outcome {
    WIN,
    DRAW,
    LOSS,
}

fn parse_outcome(s: &str) -> Result<Outcome, anyhow::Error> {
    match s {
        "X" => Ok(Outcome::LOSS),
        "Y" => Ok(Outcome::DRAW),
        "Z" => Ok(Outcome::WIN),
        _ => Err(anyhow!("couldn't convert input to Outcome: {}", s)),
    }
}

fn parse_input(input: &str) -> Result<Vec<(Choice, Outcome)>, anyhow::Error> {
    let mut output = Vec::<(Choice, Outcome)>::new();
    for line in input.split("\n") {
        let components: Vec<&str> = line.split(" ").collect();
        if components.len() < 2 {
            return Err(anyhow!("invalid number of components on line: {}", &line));
        }
        output.push((parse_choice(components[0])?, parse_outcome(components[1])?));
    }
    Ok(output)
}

fn build_game(opponent: &Choice, outcome: &Outcome) -> (Choice, Choice) {
    let player = match opponent {
        Choice::ROCK => match outcome {
            Outcome::WIN => Choice::PAPER,
            Outcome::DRAW => Choice::ROCK,
            Outcome::LOSS => Choice::SCISSORS,
        },
        Choice::PAPER => match outcome {
            Outcome::WIN => Choice::SCISSORS,
            Outcome::DRAW => Choice::PAPER,
            Outcome::LOSS => Choice::ROCK,
        },
        Choice::SCISSORS => match outcome {
            Outcome::WIN => Choice::ROCK,
            Outcome::DRAW => Choice::SCISSORS,
            Outcome::LOSS => Choice::PAPER,
        },
    };
    (opponent.clone(), player)
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
        assert_eq!((Choice::ROCK, Outcome::LOSS), result[0]);
        assert_eq!((Choice::PAPER, Outcome::DRAW), result[1]);
        assert_eq!((Choice::SCISSORS, Outcome::WIN), result[2]);
    }

    #[test]
    fn test_solve() {
        let input = "A Y
B X
C Z";
        assert_eq!("12", solve(input).expect("should return string result"));
    }
}
