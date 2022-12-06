use aoc;

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
