pub(crate) mod part_one;
pub(crate) mod part_two;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

fn score_round(opponent: &Choice, player: &Choice) -> u32 {
    // base score: 1 for rock, 2 for paper, 3 for scissors
    let base_score = match player {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };
    // add result: 0 for loss, 3 for draw, 6 for win
    match opponent {
        Choice::Rock => match player {
            Choice::Rock => base_score + 3,
            Choice::Paper => base_score + 6,
            Choice::Scissors => base_score,
        },
        Choice::Paper => match player {
            Choice::Rock => base_score,
            Choice::Paper => base_score + 3,
            Choice::Scissors => base_score + 6,
        },
        Choice::Scissors => match player {
            Choice::Rock => base_score + 6,
            Choice::Paper => base_score,
            Choice::Scissors => base_score + 3,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_round() {
        let opponent = Choice::Rock;
        let player = Choice::Scissors;
        assert_eq!(3, score_round(&opponent, &player));
    }
}
