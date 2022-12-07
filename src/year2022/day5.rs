use crate::advent_of_code::day::Day;

pub struct Day5;

impl Day5 {
    pub fn get_table(input: &str) -> (Vec<Vec<char>>, usize) {
        let mut table = Vec::new();

        'outer:
        for line in input.lines() {
            let chars: Vec<_> = line.chars().collect();

            for (count, index) in (1..chars.len()).step_by(4).enumerate() {
                if table.len() <= count {
                    table.push(Vec::new());
                }

                let char = chars.get(index).unwrap();

                if char == &' ' {
                    continue;
                }

                if char.to_string().parse::<i32>().is_ok() {
                    break 'outer;
                }

                table.get_mut(count).unwrap().push(char.to_owned());
            }
        }

        let mut highest = 0;

        for stack in &table {
            if stack.len() > highest {
                highest = stack.len();
            }
        }

        (table, highest)
    }

    pub fn do_simple_step(table: &mut [Vec<char>], line: &str) {
        let split: Vec<_> = line.split(' ').collect();

        let count = split.get(1).unwrap().parse::<usize>().unwrap();
        let from = split.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = split.get(5).unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let from = table.get_mut(from).unwrap();
            let char = from.remove(0);

            let to = table.get_mut(to).unwrap();
            to.insert(0, char)
        }
    }

    pub fn do_multi_step(table: &mut [Vec<char>], line: &str) {
        let split: Vec<_> = line.split(' ').collect();

        let count = split.get(1).unwrap().parse::<usize>().unwrap();
        let from = split.get(3).unwrap().parse::<usize>().unwrap() - 1;
        let to = split.get(5).unwrap().parse::<usize>().unwrap() - 1;

        for i in 0..count {
            let from = table.get_mut(from).unwrap();
            let char = from.remove(count - i - 1);

            let to = table.get_mut(to).unwrap();
            to.insert(0, char)
        }
    }

    pub fn get_top_of_table(table: &Vec<Vec<char>>) -> String {
        let mut text = "".to_string();

        for stack in table {
            text += &*stack.first().unwrap().to_string();
        }

        text.to_string()
    }

    pub fn calc(input: &str, use_multi: bool) -> String {
        let (mut table, highest) = Day5::get_table(input);

        let lines = input.lines().skip(highest + 2);

        for line in lines {
            if use_multi {
                Day5::do_multi_step(&mut table, line);
            } else {
                Day5::do_simple_step(&mut table, line);
            }
        }

        Day5::get_top_of_table(&table)
    }
}

impl Day for Day5 {
    fn get_path() -> String {
        "resources/year2022/day5/day1_data.txt".to_string()
    }

    fn part_1(input: &str) -> String {
        Day5::calc(input, false)
    }

    fn part_2(input: &str) -> String {
        Day5::calc(input, true)
    }
}