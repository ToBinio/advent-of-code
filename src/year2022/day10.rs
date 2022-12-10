use std::ops::Range;

use crate::advent_of_code::day::{Day, Year};

pub struct Day10;

impl Day10 {
    pub fn parse_signal(line: &str, values: &mut Vec<i32>) {
        let split: Vec<_> = line.split(' ').collect();
        let current_value = *values.last().unwrap();

        match *split.first().unwrap() {
            "addx" => {
                values.push(current_value);
                values.push(current_value + split.get(1).unwrap().parse::<i32>().unwrap());
            }

            _ => {
                values.push(current_value);
            }
        }
    }

    pub fn parse_input(input: &str) -> Vec<i32> {
        let mut values = vec![1, 1];

        for line in input.lines() {
            Day10::parse_signal(line, &mut values);
        }

        values
    }

    pub fn generate_line(values: &[i32], range: Range<usize>) -> String {
        let mut line = "".to_string();

        for (position, cycle) in range.enumerate() {
            let x = values.get(cycle).unwrap();

            if (x - position as i32).abs() <= 1 {
                line += "#"
            } else {
                line += "."
            }
        }

        line
    }
}

impl Day for Day10 {
    fn get_date() -> (Year, i32) {
        (Year::Year2022, 10)
    }

    fn part_1(input: &str) -> String {
        let values = Day10::parse_input(input);

        let mut sum = 0;

        for index in (20..values.len()).step_by(40) {
            sum += values.get(index).unwrap() * index as i32;
        }

        sum.to_string()
    }

    fn part_2(input: &str) -> String {
        let values = Day10::parse_input(input);

        let mut img = "".to_string();

        for i in (1..240).step_by(40) {
            img += Day10::generate_line(&values, i..(i + 40)).as_str();
            img += "\n";
        }

        img.pop();

        img
    }
}