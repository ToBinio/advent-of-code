use crate::advent_of_code::day::Day;

pub struct Day1;

impl Day for Day1 {
    fn get_path() -> String {
        "../../resources/year_2015/day1/data.txt".to_string()
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

    fn part_2(input: &str) -> Option<String> {
        let mut floor = 0;

        for (index, action) in input.chars().enumerate() {
            match action {
                '(' => { floor += 1 }
                ')' => { floor -= 1 }
                _ => {}
            };

            if floor < 0 {
                return Some((index + 1).to_string());
            }
        }

        Some(floor.to_string())
    }
}