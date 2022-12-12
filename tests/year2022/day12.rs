use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year2022::day12::Day12;

#[test]
fn generate_map() {
    let (map, start_pos, goal_pos) = Day12::generate_map(Day12::get_test_input().as_str());

    assert_eq!(map.len(), 5);
    assert_eq!(map.get(0).unwrap().len(), 8);

    assert_eq!(start_pos, (0, 0));
    assert_eq!(goal_pos, (2, 5));
}

#[test]
fn part_1_test() {
    assert_eq!(Day12::part_1(Day12::get_test_input().as_str()), "31")
}