use crate::advent_of_code::day::Day;

pub struct Day4;

impl Day4 {
    pub fn input_to_ranges(input: &str) -> ((i32, i32), (i32, i32)) {
        let mut input = input.split(",");
        let mut input = (input.next().unwrap().split("-"), input.next().unwrap().split("-"));
        let input = ((input.0.next().unwrap().parse().unwrap(), input.0.next().unwrap().parse().unwrap()), (input.1.next().unwrap().parse().unwrap(), input.1.next().unwrap().parse().unwrap()));

        input
    }

    pub fn in_range(point: i32, range: (i32, i32)) -> bool {
        point >= range.0 && point <= range.1
    }

    pub fn range_contains_completely(input: &str) -> bool {
        let input = Day4::input_to_ranges(input);

        (input.0.0 >= input.1.0 && input.0.1 <= input.1.1) || (input.1.0 >= input.0.0 && input.1.1 <= input.0.1)
    }

    pub fn range_contains_partly(input: &str) -> bool {
        let input = Day4::input_to_ranges(input);

        Day4::in_range(input.0.0, input.1) || Day4::in_range(input.0.1, input.1) || Day4::in_range(input.1.0, input.0) || Day4::in_range(input.1.1, input.0)
    }
}

impl Day for Day4 {
    fn get_path() -> String {
        "resources/year_2022/day4_data.txt".to_string()
    }

    fn part_1(input: &str) -> String {
        let mut count = 0;

        for line in input.lines() {
            if Day4::range_contains_completely(line) { count += 1 }
        }

        count.to_string()
    }

    fn part_2(input: &str) -> String {
        let mut count = 0;

        for line in input.lines() {
            if Day4::range_contains_partly(line) { count += 1 }
        }

        count.to_string()
    }
}