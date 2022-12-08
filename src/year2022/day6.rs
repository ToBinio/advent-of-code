use std::collections::HashSet;

use crate::advent_of_code::day::{Day, Year};

pub struct Day6;

impl Day6 {
    pub fn only_unique(input: &str) -> bool {
        let mut set = HashSet::new();

        for char in input.chars() {
            set.insert(char);
        }

        set.len() == input.len()
    }

    pub fn get_first_marker(input: &str, marker_size: usize) -> String {
        let chars: Vec<_> = input.chars().collect();

        for index in 0..chars.len() - marker_size {
            let mut string = "".to_string();

            for i in 0..marker_size {
                string += chars.get(index + i).unwrap().to_string().as_str();
            }

            if Day6::only_unique(string.as_str()) {
                return (index + marker_size).to_string();
            }
        }

        "".to_string()
    }
}

impl Day for Day6 {
    fn get_date() -> (Year, i32) {
        (Year::Year2022, 6)
    }

    fn part_1(input: &str) -> String {
        Day6::get_first_marker(input, 4)
    }

    fn part_2(input: &str) -> String {
        Day6::get_first_marker(input, 14)
    }
}