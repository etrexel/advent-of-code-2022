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
