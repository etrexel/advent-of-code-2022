pub(crate) mod part_one;
pub(crate) mod part_two;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}

fn score_round(opponent: &Choice, player: &Choice) -> u32 {
    // base score: 1 for rock, 2 for paper, 3 for scissors
    let base_score = match player {
        Choice::ROCK => 1,
        Choice::PAPER => 2,
        Choice::SCISSORS => 3,
    };
    // add result: 0 for loss, 3 for draw, 6 for win
    match opponent {
        Choice::ROCK => match player {
            Choice::ROCK => base_score + 3,
            Choice::PAPER => base_score + 6,
            Choice::SCISSORS => base_score + 0,
        },
        Choice::PAPER => match player {
            Choice::ROCK => base_score + 0,
            Choice::PAPER => base_score + 3,
            Choice::SCISSORS => base_score + 6,
        },
        Choice::SCISSORS => match player {
            Choice::ROCK => base_score + 6,
            Choice::PAPER => base_score + 0,
            Choice::SCISSORS => base_score + 3,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_round() {
        let opponent = Choice::ROCK;
        let player = Choice::SCISSORS;
        assert_eq!(3, score_round(&opponent, &player));
    }
}
