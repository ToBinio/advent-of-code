use std::collections::HashSet;

use crate::advent_of_code::day::{Day, Year};

pub struct Day9;

impl Day9 {
    pub fn get_movement(movement: &str) -> ((i32, i32), i32) {
        let mut split = movement.split(' ');

        let dir = match split.next().unwrap() {
            "R" => { (1, 0) }
            "L" => { (-1, 0) }
            "U" => { (0, 1) }
            "D" => { (0, -1) }
            _ => { (0, 0) }
        };

        (dir, split.next().unwrap().parse().unwrap())
    }

    pub fn move_tail(tail_pos: &mut (i32, i32), head_pos: &(i32, i32)) {
        let mut offset = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);

        if offset.0.abs() < 2 && offset.1.abs() < 2 {
            return;
        }

        if offset.0.abs() == 1 {
            offset.0 *= 2;
        }

        if offset.1.abs() == 1 {
            offset.1 *= 2;
        }

        tail_pos.0 += offset.0 / 2;
        tail_pos.1 += offset.1 / 2;
    }

    pub fn do_move(segments: &mut Vec<(i32, i32)>, movement: &str, set: &mut HashSet<(i32, i32)>) {
        let movement = Day9::get_movement(movement);

        for _ in 0..movement.1 {
            let head_pos = segments.first_mut().unwrap();
            head_pos.0 += movement.0.0;
            head_pos.1 += movement.0.1;

            for i in 0..(segments.len() - 1) {
                let segment1 = *segments.get(i).unwrap();
                let segment2 = segments.get_mut(i + 1).unwrap();

                Day9::move_tail(segment2, &segment1);
            }

            let tail_pos = segments.last().unwrap();

            set.insert((tail_pos.0, tail_pos.1));
        }
    }

    pub fn compute_tail(input: &str, size: usize) -> usize {
        let mut segments = vec![(0, 0); size];

        let mut set = HashSet::new();

        for line in input.lines() {
            Day9::do_move(&mut segments, line, &mut set);
        }

        set.len()
    }
}

impl Day for Day9 {
    fn get_date() -> (Year, i32) {
        (Year::Year2022, 9)
    }

    fn part_1(input: &str) -> String {
        Day9::compute_tail(input, 2).to_string()
    }

    fn part_2(input: &str) -> String {
        Day9::compute_tail(input, 10).to_string()
    }
}