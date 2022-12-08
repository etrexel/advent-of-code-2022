// TODO: docs

use crate::solution::day_06::decode_bitstream;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    Ok(decode_bitstream(input, 4).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!("7", solve(input).expect("should return result"));
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!("5", solve(input).expect("should return result"));
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!("6", solve(input).expect("should return result"));
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!("10", solve(input).expect("should return result"));
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!("11", solve(input).expect("should return result"));
    }
}
