use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year_2022::day_2::Day2;

#[test]
fn part_1_test() {
    assert_eq!(Day2::part_1(include_str!("../../resources/year_2022/day2/test.txt")), "15");
}

#[test]
fn get_result() {
    assert_eq!(Day2::get_result(("A", "X")), 3);
    assert_eq!(Day2::get_result(("B", "Y")), 3);
    assert_eq!(Day2::get_result(("C", "Z")), 3);

    assert_eq!(Day2::get_result(("A", "Z")), 0);
    assert_eq!(Day2::get_result(("B", "X")), 0);
    assert_eq!(Day2::get_result(("C", "Y")), 0);

    assert_eq!(Day2::get_result(("A", "Y")), 6);
    assert_eq!(Day2::get_result(("B", "Z")), 6);
    assert_eq!(Day2::get_result(("C", "X")), 6);
}

#[test]
fn part_2_test() {
    assert_eq!(Day2::part_2(include_str!("../../resources/year_2022/day2/test.txt")).unwrap(), "12");
}

#[test]
fn get_what_to_choose() {
    assert_eq!(Day2::get_what_to_choose(("B", "Y")), "B");
    assert_eq!(Day2::get_what_to_choose(("A", "X")), "C");
    assert_eq!(Day2::get_what_to_choose(("C", "Z")), "A");

    assert_eq!(Day2::get_what_to_choose(("C", "Y")), "C");
    assert_eq!(Day2::get_what_to_choose(("B", "X")), "A");
    assert_eq!(Day2::get_what_to_choose(("A", "Z")), "B");

    assert_eq!(Day2::get_what_to_choose(("A", "Y")), "A");
    assert_eq!(Day2::get_what_to_choose(("C", "X")), "B");
    assert_eq!(Day2::get_what_to_choose(("B", "Z")), "C");
}