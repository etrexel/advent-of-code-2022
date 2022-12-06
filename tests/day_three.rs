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
