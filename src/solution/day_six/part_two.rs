use crate::solution::day_six::decode_bitstream;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    Ok(decode_bitstream(input, 14).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!("19", solve(input).expect("should return result"));
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!("23", solve(input).expect("should return result"));
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!("23", solve(input).expect("should return result"));
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!("29", solve(input).expect("should return result"));
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!("26", solve(input).expect("should return result"));
    }
}
