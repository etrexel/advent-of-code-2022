use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli() {
    let mut cmd = Command::cargo_bin("aoc").expect("should create command");
    cmd.arg("-d").arg("1").arg("-p").arg("1");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("69626"));
}

#[test]
fn test_cli_error() {
    let mut cmd = Command::cargo_bin("aoc").expect("should create command");
    cmd.arg("-d").arg("0");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("ERROR"));
}

#[test]
fn test_day_01_part_1() {
    assert_eq!(
        "69626",
        aoc::solve(1, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_01_part_2() {
    assert_eq!(
        "206780",
        aoc::solve(1, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_02_part_1() {
    assert_eq!(
        "12535",
        aoc::solve(2, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_02_part_2() {
    assert_eq!(
        "15457",
        aoc::solve(2, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_03_part_1() {
    assert_eq!(
        "7821",
        aoc::solve(3, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_03_part_2() {
    assert_eq!(
        "2752",
        aoc::solve(3, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_04_part_1() {
    assert_eq!("567", aoc::solve(4, 1, None).expect("should return result"));
}

#[test]
fn test_day_04_part_2() {
    assert_eq!("907", aoc::solve(4, 2, None).expect("should return result"));
}

#[test]
fn test_day_05_part_1() {
    assert_eq!(
        "SVFDLGLWV",
        aoc::solve(5, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_05_part_2() {
    assert_eq!(
        "DCVTCVPCL",
        aoc::solve(5, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_06_part_1() {
    assert_eq!(
        "1855",
        aoc::solve(6, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_06_part_2() {
    assert_eq!(
        "3256",
        aoc::solve(6, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_07_part_1() {
    assert_eq!(
        "1444896",
        aoc::solve(7, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_07_part_2() {
    assert_eq!(
        "404395",
        aoc::solve(7, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_08_part_1() {
    assert_eq!(
        "1782",
        aoc::solve(8, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_08_part_2() {
    assert_eq!(
        "474606",
        aoc::solve(8, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_09_part_1() {
    assert_eq!(
        "5907",
        aoc::solve(9, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_09_part_2() {
    assert_eq!(
        "2303",
        aoc::solve(9, 2, None).expect("should return result")
    );
}

#[test]
fn test_day_10_part_1() {
    assert_eq!(
        "14860",
        aoc::solve(10, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_10_part_2() {
    let gt = "
###...##..####.####.#..#.#..#.###..#..#.
#..#.#..#....#.#....#..#.#..#.#..#.#.#..
#..#.#......#..###..####.#..#.#..#.##...
###..#.##..#...#....#..#.#..#.###..#.#..
#.#..#..#.#....#....#..#.#..#.#.#..#.#..
#..#..###.####.####.#..#..##..#..#.#..#.\n";
    assert_eq!(gt, aoc::solve(10, 2, None).expect("should return result"));
}

#[test]
fn test_day_11_part_1() {
    assert_eq!(
        "95472",
        aoc::solve(11, 1, None).expect("should return result")
    );
}

#[test]
fn test_day_11_part_2() {
    assert_eq!(
        "17926061332",
        aoc::solve(11, 2, None).expect("should return result")
    );
}
