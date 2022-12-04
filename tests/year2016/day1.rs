use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year2016::day1::Day1;

#[test]
fn part_1_test() {
    assert_eq!(Day1::part_1("R2, L3"), "5");
    assert_eq!(Day1::part_1("R2, R2, R2"), "2");
    assert_eq!(Day1::part_1("R5, L5, R5, R3"), "12");
}

#[test]
fn part_2_test() {
    assert_eq!(Day1::part_2("R8, R4, R4, R8"), "4");
}