use std::f32::consts::PI;

use crate::advent_of_code::day::{Day, Year};

pub struct Day1;

impl Day for Day1 {
    fn get_date() -> (Year, i32) {
        (Year::Year2016, 1)
    }

    fn part_1(input: &str) -> String {
        let mut dir: f32 = 0.0;
        let mut pos = (0, 0);

        for step in input.split(',') {
            let step = step.trim().to_string();

            match &step[0..1] {
                "R" => {
                    dir += PI / 2.0;
                }

                _ => {
                    dir -= PI / 2.0;
                }
            }

            let mul = step[1..step.len()].parse::<f32>().unwrap();

            pos.0 += (dir.cos().round() * mul) as i32;
            pos.1 += (dir.sin().round() * mul) as i32;
        };

        (pos.0.abs() + pos.1.abs()).to_string()
    }

    fn part_2(input: &str) -> String {
        let mut dir: f32 = 0.0;
        let mut poses = Vec::new();
        let mut pos = (0, 0);

        'outer:
        for step in input.split(',') {
            let step = step.trim().to_string();

            match &step[0..1] {
                "R" => {
                    dir += PI / 2.0;
                }

                _ => {
                    dir -= PI / 2.0;
                }
            }

            let mul = step[1..step.len()].parse::<f32>().unwrap() as i32;

            for _ in 0..mul {
                pos.0 += dir.cos().round() as i32;
                pos.1 += dir.sin().round() as i32;

                if poses.contains(&pos) {
                    break 'outer;
                }

                poses.push(pos);
            }
        };

        (pos.0.abs() + pos.1.abs()).to_string()
    }
}