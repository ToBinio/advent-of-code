use std::collections::HashSet;

use crate::advent_of_code::day::{Day, Year};

pub struct Day3;

impl Day3 {
    pub fn calc_step(mut pos: (i32, i32), step: char) -> (i32, i32) {
        match step {
            '<' => { pos.0 -= 1; }
            '>' => { pos.0 += 1; }
            '^' => { pos.1 += 1; }
            'v' => { pos.1 -= 1; }
            _ => {}
        }

        pos
    }
}

impl Day for Day3 {
    fn get_date() -> (Year, i32) {
        (Year::Year2015, 3)
    }

    fn part_1(input: &str) -> String {
        let mut map = HashSet::new();

        let mut pos = (0, 0);
        map.insert(pos);

        for step in input.chars() {
            pos = Day3::calc_step(pos, step);
            map.insert(pos);
        }

        map.len().to_string()
    }

    fn part_2(input: &str) -> String {
        let mut map = HashSet::new();

        let mut santa_pos = (0, 0);
        let mut robot_pos = (0, 0);

        map.insert(santa_pos);

        for (index, step) in input.chars().enumerate() {
            if index % 2 == 0 {
                santa_pos = Day3::calc_step(santa_pos, step);
                map.insert(santa_pos);
            } else {
                robot_pos = Day3::calc_step(robot_pos, step);
                map.insert(robot_pos);
            }
        }

        map.len().to_string()
    }
}