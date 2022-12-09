use std::collections::HashSet;

use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year2022::day9::Day9;

#[test]
fn do_move_test() {

    let mut segments = vec![(0, 0); 2];
    
    Day9::do_move(&mut segments, "R 4", &mut HashSet::new());
    assert_eq!(*segments.first().unwrap(), (4, 0));
    assert_eq!(*segments.last().unwrap(), (3, 0));

    Day9::do_move(&mut segments, "U 4", &mut HashSet::new());
    assert_eq!(*segments.first().unwrap(), (4, 4));
    assert_eq!(*segments.last().unwrap(), (4, 3));

    Day9::do_move(&mut segments, "L 3", &mut HashSet::new());
    assert_eq!(*segments.first().unwrap(), (1, 4));
    assert_eq!(*segments.last().unwrap(), (2, 4));

    Day9::do_move(&mut segments, "D 1", &mut HashSet::new());
    assert_eq!(*segments.first().unwrap(), (1, 3));
    assert_eq!(*segments.last().unwrap(), (2, 4));

    Day9::do_move(&mut segments, "R 4", &mut HashSet::new());
    assert_eq!(*segments.first().unwrap(), (5, 3));
    assert_eq!(*segments.last().unwrap(), (4, 3));

    Day9::do_move(&mut segments, "D 1", &mut HashSet::new());
    assert_eq!(*segments.first().unwrap(), (5, 2));
    assert_eq!(*segments.last().unwrap(), (4, 3));

    Day9::do_move(&mut segments, "L 5", &mut HashSet::new());
    assert_eq!(*segments.first().unwrap(), (0, 2));
    assert_eq!(*segments.last().unwrap(), (1, 2));

    Day9::do_move(&mut segments, "R 2", &mut HashSet::new());
    assert_eq!(*segments.first().unwrap(), (2, 2));
    assert_eq!(*segments.last().unwrap(), (1, 2));
}

#[test]
fn part_1_test() {
    assert_eq!(Day9::part_1(Day9::get_test_input().as_str()), "13")
}

#[test]
fn part_2_test() {
    assert_eq!(Day9::part_2(Day9::get_test_input().as_str()), "1");
    assert_eq!(Day9::part_2(include_str!("../../resources/year2022/day9_test2.txt")), "36");
}