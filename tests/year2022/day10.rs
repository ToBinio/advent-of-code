use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year2022::day10::Day10;

#[test]
fn parse_input_test() {
    let values = Day10::parse_input(Day10::get_test_input().as_str());

    assert_eq!(*values.get(20).unwrap(), 21);
    assert_eq!(*values.get(60).unwrap(), 19);
    assert_eq!(*values.get(100).unwrap(), 18);
    assert_eq!(*values.get(140).unwrap(), 21);
    assert_eq!(*values.get(180).unwrap(), 16);
    assert_eq!(*values.get(220).unwrap(), 18);
}

#[test]
fn generate_line_test() {
    let values = Day10::parse_input(Day10::get_test_input().as_str());

    assert_eq!(Day10::generate_line(&values, 1..41), "##..##..##..##..##..##..##..##..##..##..");
    assert_eq!(Day10::generate_line(&values, 41..81), "###...###...###...###...###...###...###.");
    assert_eq!(Day10::generate_line(&values, 81..121), "####....####....####....####....####....");
    assert_eq!(Day10::generate_line(&values, 121..161), "#####.....#####.....#####.....#####.....");
    assert_eq!(Day10::generate_line(&values, 161..201), "######......######......######......####");
    assert_eq!(Day10::generate_line(&values, 201..241), "#######.......#######.......#######.....");
}

#[test]
fn part_1_test() {
    assert_eq!(Day10::part_1(Day10::get_test_input().as_str()), "13140")
}

#[test]
fn part_2_test() {
    assert_eq!(Day10::part_2(Day10::get_test_input().as_str()),
               "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....")
}
