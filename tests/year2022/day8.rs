use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year2022::day8::Day8;

#[test]
fn to_grid_test() {
    let grid = Day8::make_grid(include_str!("../../resources/year2022/day8/test.txt"));

    assert_eq!(grid.len(), 5);

    for line in &grid {
        assert_eq!(line.len(), 5);
    }
}

#[test]
fn is_visible_test() {
    let grid = Day8::make_grid(include_str!("../../resources/year2022/day8/test.txt"));

    assert!(Day8::is_visible((0, 0), &grid));
    assert!(Day8::is_visible((2, 4), &grid));
    assert!(Day8::is_visible((4, 0), &grid));

    assert!(Day8::is_visible((1, 1), &grid));
    assert!(Day8::is_visible((2, 1), &grid));
    assert!(!Day8::is_visible((3, 1), &grid));
    assert!(Day8::is_visible((1, 2), &grid));
    assert!(!Day8::is_visible((2, 2), &grid));
    assert!(Day8::is_visible((3, 2), &grid));
    assert!(!Day8::is_visible((1, 3), &grid));
    assert!(Day8::is_visible((2, 3), &grid));
    assert!(!Day8::is_visible((3, 3), &grid));
}

#[test]
fn get_scenic_score_test() {
    let grid = Day8::make_grid(include_str!("../../resources/year2022/day8/test.txt"));
    assert_eq!(Day8::get_scenic_score((2, 1), &grid), 4);
    assert_eq!(Day8::get_scenic_score((2, 3), &grid), 8);
}

#[test]
fn part_1_test() {
    assert_eq!(Day8::part_1(include_str!("../../resources/year2022/day8/test.txt")), "21")
}


#[test]
fn part_2_test() {
    assert_eq!(Day8::part_2(include_str!("../../resources/year2022/day8/test.txt")), "8")
}