use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year_2015::day1::Day1;

#[test]
fn part_1_test() {
    assert_eq!(Day1::part_1("(())"), "0");
    assert_eq!(Day1::part_1("()()"), "0");

    assert_eq!(Day1::part_1("((("), "3");
    assert_eq!(Day1::part_1("(()(()("), "3");
    assert_eq!(Day1::part_1("))((((("), "3");

    assert_eq!(Day1::part_1("())"), "-1");
    assert_eq!(Day1::part_1("))("), "-1");

    assert_eq!(Day1::part_1(")))"), "-3");
    assert_eq!(Day1::part_1(")())())"), "-3");
}

#[test]
fn part_2_test() {
    assert_eq!(Day1::part_2(")").unwrap(), "1");
    assert_eq!(Day1::part_2("()())").unwrap(), "5");
}