use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year2022::day7::Day7;

#[test]
fn parse_test() {
    let root = Day7::parse(include_str!("../../resources/year2022/day7_test.txt"));

    assert_eq!(root.get_size(), 48381165);

    let ad = root.get_inner_folders();
    let (a, d) = (ad.get(0).unwrap(), ad.get(1).unwrap());

    assert_eq!(a.get_name(), "a".to_string());
    assert_eq!(a.get_size(), 94853);
    assert_eq!(d.get_name(), "d".to_string());
    assert_eq!(d.get_size(), 24933642);

    let e = a.get_inner_folders().get(0).unwrap();

    assert_eq!(e.get_name(), "e".to_string());
    assert_eq!(e.get_size(), 584);
}

#[test]
fn part_1_test() {
    assert_eq!(Day7::part_1(include_str!("../../resources/year2022/day7_test.txt")), "95437");
}

#[test]
fn part_2_test() {
    assert_eq!(Day7::part_2(include_str!("../../resources/year2022/day7_test.txt")), "24933642");
}