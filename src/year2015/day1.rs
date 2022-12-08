use crate::advent_of_code::day::{Day, Year};

pub struct Day1;

impl Day for Day1 {
    fn get_date() -> (Year, i32) {
        (Year::Year2015, 1)
    }

    fn part_1(input: &str) -> String {
        let mut floor = 0;

        input.chars()
            .for_each(|action| {
                match action {
                    '(' => { floor += 1 }
                    ')' => { floor -= 1 }
                    _ => {}
                };
            });

        floor.to_string()
    }

    fn part_2(input: &str) -> String {
        let mut floor = 0;

        for (index, action) in input.chars().enumerate() {
            match action {
                '(' => { floor += 1 }
                ')' => { floor -= 1 }
                _ => {}
            };

            if floor < 0 {
                return (index + 1).to_string();
            }
        }

        floor.to_string()
    }
}