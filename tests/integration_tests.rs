#[test]
fn test_day_one_part_one() {
    assert_eq!(
        "69626",
        aoc::solve(1, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_one_part_two() {
    assert_eq!(
        "206780",
        aoc::solve(1, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_two_part_one() {
    assert_eq!(
        "12535",
        aoc::solve(2, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_two_part_two() {
    assert_eq!(
        "15457",
        aoc::solve(2, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_three_part_one() {
    assert_eq!(
        "7821",
        aoc::solve(3, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_three_part_two() {
    assert_eq!(
        "2752",
        aoc::solve(3, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_four_part_one() {
    assert_eq!("567", aoc::solve(4, 1, None).expect("should return result"));
}

#[test]
fn test_day_four_part_two() {
    assert_eq!("907", aoc::solve(4, 2, None).expect("should return result"));
}

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

#[test]
fn test_day_six_part_one() {
    assert_eq!(
        "1855",
        aoc::solve(6, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_six_part_two() {
    assert_eq!(
        "3256",
        aoc::solve(6, 2, None).expect("should return result")
    );
}
