use crate::advent_of_code::day::Day;
use crate::advent_of_code::input;

pub struct Day3;

impl Day3 {
    pub fn get_shared_item(input: &str) -> char {
        let middle = input.len() / 2;
        let input = (&input[0..middle], &input[middle..input.len()]);

        let option: Vec<char> = input.0.chars().filter(|char| input.1.contains(char.to_string().as_str())).collect();

        *option.first().unwrap()
    }

    pub fn get_group_item(input1: &str, input2: &str, input3: &str) -> char {
        let option: Vec<char> = input1.chars()
            .filter(|char| input2.contains(char.to_string().as_str()))
            .filter(|char| input3.contains(char.to_string().as_str()))
            .collect();

        *option.first().unwrap()
    }

    pub fn get_priority(char: char) -> i32 {
        let char = char as i32;

        if char >= 97 {
            char - 96
        } else {
            char - 64 + 26
        }
    }
}

impl Day for Day3 {
    fn get_path() -> String {
        "resources/year2022/day3_data.txt".to_string()
    }

    fn part_1(input: &str) -> String {
        input::sum(input, |line| Day3::get_priority(Day3::get_shared_item(line))).to_string()
    }

    fn part_2(input: &str) -> String {
        let mut sum = 0;
        let lines: Vec<&str> = input.lines().collect();

        for index in (0..lines.len()).step_by(3) {
            sum += Day3::get_priority(Day3::get_group_item(lines.get(index).unwrap(), lines.get(index + 1).unwrap(), lines.get(index + 2).unwrap()));
        }

        sum.to_string()
    }
}