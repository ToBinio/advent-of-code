use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year2022::day6::Day6;

#[test]
fn only_unique_test() {
    assert!(Day6::only_unique("abcd"));
    assert!(Day6::only_unique("kzgd"));
    assert!(!Day6::only_unique("kdud"));
    assert!(!Day6::only_unique("accd"));
}

#[test]
fn part_1_test() {
    assert_eq!(Day6::part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7");
    assert_eq!(Day6::part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
    assert_eq!(Day6::part_1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
    assert_eq!(Day6::part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
    assert_eq!(Day6::part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
}

#[test]
fn part_2_test() {
    assert_eq!(Day6::part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
    assert_eq!(Day6::part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
    assert_eq!(Day6::part_2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
    assert_eq!(Day6::part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
    assert_eq!(Day6::part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
}