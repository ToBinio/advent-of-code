use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year2022::day5::Day5;

#[test]
fn get_table_test() {
    let (table, count) = Day5::get_table(include_str!("../../resources/year2022/day5/test.txt"));

    assert_eq!(*table.get(0).unwrap(), vec!['N', 'Z']);
    assert_eq!(*table.get(1).unwrap(), vec!['D', 'C', 'M']);
    assert_eq!(*table.get(2).unwrap(), vec!['P']);

    assert_eq!(count, 3);
}

#[test]
fn do_step_simple_test() {
    let (mut table, _) = Day5::get_table(include_str!("../../resources/year2022/day5/test.txt"));

    Day5::do_simple_step(&mut table, "move 1 from 2 to 1");

    assert_eq!(*table.get(0).unwrap(), vec!['D', 'N', 'Z']);
    assert_eq!(*table.get(1).unwrap(), vec!['C', 'M']);
    assert_eq!(*table.get(2).unwrap(), vec!['P']);

    Day5::do_simple_step(&mut table, "move 3 from 1 to 3");

    assert_eq!(*table.get(0).unwrap(), vec![]);
    assert_eq!(*table.get(1).unwrap(), vec!['C', 'M']);
    assert_eq!(*table.get(2).unwrap(), vec!['Z', 'N', 'D', 'P']);

    Day5::do_simple_step(&mut table, "move 2 from 2 to 1");

    assert_eq!(*table.get(0).unwrap(), vec!['M', 'C']);
    assert_eq!(*table.get(1).unwrap(), vec![]);
    assert_eq!(*table.get(2).unwrap(), vec!['Z', 'N', 'D', 'P']);

    Day5::do_simple_step(&mut table, "move 1 from 1 to 2");

    assert_eq!(*table.get(0).unwrap(), vec!['C']);
    assert_eq!(*table.get(1).unwrap(), vec!['M']);
    assert_eq!(*table.get(2).unwrap(), vec!['Z', 'N', 'D', 'P']);
}


#[test]
fn do_multi_step_test() {
    let (mut table, _) = Day5::get_table(include_str!("../../resources/year2022/day5/test.txt"));

    Day5::do_multi_step(&mut table, "move 1 from 2 to 1");

    assert_eq!(*table.get(0).unwrap(), vec!['D', 'N', 'Z']);
    assert_eq!(*table.get(1).unwrap(), vec!['C', 'M']);
    assert_eq!(*table.get(2).unwrap(), vec!['P']);

    Day5::do_multi_step(&mut table, "move 3 from 1 to 3");

    assert_eq!(*table.get(0).unwrap(), vec![]);
    assert_eq!(*table.get(1).unwrap(), vec!['C', 'M']);
    assert_eq!(*table.get(2).unwrap(), vec!['D', 'N', 'Z', 'P']);

    Day5::do_multi_step(&mut table, "move 2 from 2 to 1");

    assert_eq!(*table.get(0).unwrap(), vec!['C', 'M']);
    assert_eq!(*table.get(1).unwrap(), vec![]);
    assert_eq!(*table.get(2).unwrap(), vec!['D', 'N', 'Z', 'P']);

    Day5::do_multi_step(&mut table, "move 1 from 1 to 2");

    assert_eq!(*table.get(0).unwrap(), vec!['M']);
    assert_eq!(*table.get(1).unwrap(), vec!['C']);
    assert_eq!(*table.get(2).unwrap(), vec!['D', 'N', 'Z', 'P']);
}

#[test]
fn get_top_of_table_test() {
    let (table, _) = Day5::get_table(include_str!("../../resources/year2022/day5/test.txt"));
    assert_eq!(Day5::get_top_of_table(&table), "NDP")
}

#[test]
fn part_1_test() {
    assert_eq!(Day5::part_1(include_str!("../../resources/year2022/day5/test.txt")), "CMZ")
}

#[test]
fn part_2_test() {
    assert_eq!(Day5::part_2(include_str!("../../resources/year2022/day5/test.txt")), "MCD")
}