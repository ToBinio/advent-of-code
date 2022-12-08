use advent_of_code::year2022::day3::Day3;

#[test]
fn get_shared_item_test() {
    assert_eq!(Day3::get_shared_item("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
    assert_eq!(Day3::get_shared_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
    assert_eq!(Day3::get_shared_item("PmmdzqPrVvPwwTWBwg"), 'P');
    assert_eq!(Day3::get_shared_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 'v');
    assert_eq!(Day3::get_shared_item("ttgJtRGJQctTZtZT"), 't');
    assert_eq!(Day3::get_shared_item("CrZsJsPPZsGzwwsLwLmpwMDw"), 's');
}

#[test]
fn get_group_item_test() {
    assert_eq!(Day3::get_group_item("vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg"), 'r');
    assert_eq!(Day3::get_group_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"), 'Z');
}

#[test]
fn get_priority_test() {
    assert_eq!(Day3::get_priority('p'), 16);
    assert_eq!(Day3::get_priority('L'), 38);
    assert_eq!(Day3::get_priority('P'), 42);
    assert_eq!(Day3::get_priority('v'), 22);
    assert_eq!(Day3::get_priority('t'), 20);
    assert_eq!(Day3::get_priority('s'), 19);
}