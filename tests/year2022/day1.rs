use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year2022::day1::Day1;

#[test]
fn part_1_test() {
    assert_eq!(Day1::part_1(Day1::get_test_input().as_str()), "24000")
}

#[test]
fn part_2_test() {
    assert_eq!(Day1::part_2(Day1::get_test_input().as_str()), "45000")
}