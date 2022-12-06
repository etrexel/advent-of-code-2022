#[test]
fn test_day_five_part_one() {
    assert_eq!(
        "SVFDLGLWV",
        aoc::solve(5, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_five_part_two() {
    assert_eq!(
        "DCVTCVPCL",
        aoc::solve(5, 2, None).expect("should return result")
    );
}
