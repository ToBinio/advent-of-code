use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year_2015::day3::Day3;

#[test]
fn part_1_test() {
    assert_eq!(Day3::part_1(">"), "2");
    assert_eq!(Day3::part_1("^>v<"), "4");
    assert_eq!(Day3::part_1("^v^v^v^v^v"), "2");
}

#[test]
fn part_2_test() {
    assert_eq!(Day3::part_2("^v"), "3");
    assert_eq!(Day3::part_2("^>v<"), "3");
    assert_eq!(Day3::part_2("^v^v^v^v^v"), "11");
}