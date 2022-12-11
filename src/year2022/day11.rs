use crate::advent_of_code::day::{Day, Year};
use crate::year2022::day11::Value::{Number, Old};

pub struct Day11;

impl Day11 {
    pub fn round(monkeys: &mut Vec<Monkey>, super_mod: i64, div_3: bool) {
        for index in 0..monkeys.len() {
            let monkey = monkeys.get_mut(index).unwrap();

            let mut items = (vec![], vec![]);

            for item in &monkey.items {
                let mut item = *item;
                item = monkey.operation.calc(item);

                if div_3 {
                    item /= 3;
                }

                item %= super_mod;

                monkey.inspection_count += 1;

                if item % monkey.test_value == 0 {
                    items.0.push(item.to_owned());
                } else {
                    items.1.push(item.to_owned());
                }
            }

            let test_result_true = monkey.test_result_true as usize;
            let test_result_false = monkey.test_result_false as usize;

            monkeys.get_mut(index).unwrap().items.clear();

            monkeys.get_mut(test_result_true).unwrap().items.append(&mut items.0);
            monkeys.get_mut(test_result_false).unwrap().items.append(&mut items.1);
        }
    }
}

impl Day for Day11 {
    fn get_date() -> (Year, i32) {
        (Year::Year2022, 11)
    }

    fn part_1(input: &str) -> String {
        let (mut monkeys, super_mod) = Monkey::parse_monkeys(input);
        for _ in 0..20 {
            Day11::round(&mut monkeys, super_mod, true);
        }

        monkeys.sort_by(|monkey1, monkey2| monkey1.inspection_count.cmp(&monkey2.inspection_count).reverse());

        (monkeys.first().unwrap().inspection_count * monkeys.get(1).unwrap().inspection_count).to_string()
    }

    fn part_2(input: &str) -> String {
        let (mut monkeys, super_mod) = Monkey::parse_monkeys(input);
        for _ in 0..10000 {
            Day11::round(&mut monkeys, super_mod, false);
        }

        monkeys.sort_by(|monkey1, monkey2| monkey1.inspection_count.cmp(&monkey2.inspection_count).reverse());

        (monkeys.first().unwrap().inspection_count as i128 * monkeys.get(1).unwrap().inspection_count as i128).to_string()
    }
}

pub struct Monkey {
    pub items: Vec<i64>,
    pub operation: Operation,
    pub test_value: i64,
    pub test_result_true: i64,
    pub test_result_false: i64,
    pub inspection_count: i64,
}

impl Monkey {
    pub fn parse_monkeys(input: &str) -> (Vec<Monkey>, i64) {
        let mut monkeys = vec![];
        let mut super_mod = 1;

        for monkey in input.split("\r\n\r\n") {
            let monkey = Monkey::parse(monkey);
            super_mod *= monkey.test_value;
            monkeys.push(monkey);
        };

        (monkeys, super_mod)
    }

    pub fn parse(input: &str) -> Monkey {
        let mut lines = input.lines().skip(1);
        let items = lines.next().unwrap().split(':').nth(1).unwrap().split(',');
        let items: Vec<_> = items.map(|item| item.trim().parse::<i64>().unwrap()).collect();

        let operation = Operation::parse(&lines.next().unwrap().split(':').nth(1).unwrap().trim()[6..]);

        let test_value = lines.next().unwrap().trim().split(' ').nth(3).unwrap().parse::<i64>().unwrap();

        let test_result_true = lines.next().unwrap().trim().split(' ').nth(5).unwrap().parse::<i64>().unwrap();
        let test_result_false = lines.next().unwrap().trim().split(' ').nth(5).unwrap().parse::<i64>().unwrap();

        Monkey {
            items,
            operation,
            test_value,
            test_result_true,
            test_result_false,
            inspection_count: 0,
        }
    }
}

pub enum Value {
    Number(i64),
    Old,
}

pub struct Operation {
    first_value: Value,
    second_value: Value,

    operation: String,
}

impl Operation {
    pub fn parse(input: &str) -> Operation {
        let mut input = input.trim().split(' ');
        Operation {
            first_value: Self::parse_value(input.next().unwrap()),
            operation: input.next().unwrap().to_string(),
            second_value: Self::parse_value(input.next().unwrap()),
        }
    }

    pub fn parse_value(input: &str) -> Value {
        match input {
            "old" => { Old }
            _ => { Number(input.parse().unwrap()) }
        }
    }

    pub fn calc(&self, value: i64) -> i64 {
        let first_value = match self.first_value {
            Number(number) => { number }
            Old => { value }
        };

        let second_value = match self.second_value {
            Number(number) => { number }
            Old => { value }
        };

        match self.operation.as_str() {
            "+" => { first_value + second_value }
            "*" => { first_value * second_value }
            _ => { 0 }
        }
    }
}