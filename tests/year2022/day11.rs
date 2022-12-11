use advent_of_code::advent_of_code::day::Day;
use advent_of_code::year2022::day11::{Day11, Monkey, Operation};

#[test]
fn operation_test() {
    let operation = Operation::parse("old * 19");
    assert_eq!(operation.calc(20), 20 * 19);
    assert_eq!(operation.calc(5), 5 * 19);

    let operation = Operation::parse("old + 6");
    assert_eq!(operation.calc(20), 20 + 6);
    assert_eq!(operation.calc(5), 5 + 6);

    let operation = Operation::parse("old * old");
    assert_eq!(operation.calc(20), 20 * 20);
    assert_eq!(operation.calc(5), 5 * 5);

    let operation = Operation::parse("old + 3");
    assert_eq!(operation.calc(20), 20 + 3);
    assert_eq!(operation.calc(5), 5 + 3);
}

#[test]
fn monkey_test() {
    let monkey = Monkey::parse(
        "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3");

    assert_eq!(monkey.operation.calc(5), Operation::parse("old * 19").calc(5));
    assert_eq!(monkey.test_value, 23);
    assert_eq!(monkey.test_result_true, 2);
    assert_eq!(monkey.test_result_false, 3);
}


#[test]
fn round_test() {
    let (mut monkeys, super_mod) = Monkey::parse_monkeys(Day11::get_test_input().as_str());

    assert_eq!(monkeys.len(), 4);

    Day11::round(&mut monkeys, super_mod, true);

    assert_eq!(monkeys.get(0).unwrap().items.len(), 4);
    assert_eq!(monkeys.get(1).unwrap().items.len(), 6);
    assert_eq!(monkeys.get(2).unwrap().items.len(), 0);
    assert_eq!(monkeys.get(3).unwrap().items.len(), 0);

    Day11::round(&mut monkeys, super_mod, true);

    assert_eq!(monkeys.get(0).unwrap().items.len(), 5);
    assert_eq!(monkeys.get(1).unwrap().items.len(), 5);
    assert_eq!(monkeys.get(2).unwrap().items.len(), 0);
    assert_eq!(monkeys.get(3).unwrap().items.len(), 0);

    for _ in 2..20 {
        Day11::round(&mut monkeys, super_mod, true);
    }

    assert_eq!(monkeys.get(0).unwrap().inspection_count, 101);
    assert_eq!(monkeys.get(1).unwrap().inspection_count, 95);
    assert_eq!(monkeys.get(2).unwrap().inspection_count, 7);
    assert_eq!(monkeys.get(3).unwrap().inspection_count, 105);
}

#[test]
fn part_1_test() {
    assert_eq!(Day11::part_1(Day11::get_test_input().as_str()), "10605");
}

#[test]
fn part_2_test() {
    assert_eq!(Day11::part_2(Day11::get_test_input().as_str()), "2713310158");
}