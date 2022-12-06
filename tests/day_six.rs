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
