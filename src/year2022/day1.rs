use crate::advent_of_code::day::{Day, Year};

pub struct Day1;

impl Day1 {
    pub fn generate_elves_calories(input: &str) -> Vec<i32> {
        let mut elves_calories = Vec::new();

        let mut sum = 0;

        for line in input.lines() {
            if line.is_empty() {
                elves_calories.push(sum);
                sum = 0;
            } else {
                sum += line.parse::<i32>().unwrap();
            }
        }

        elves_calories.push(sum);

        elves_calories.sort();
        elves_calories.reverse();

        elves_calories
    }
}

impl Day for Day1 {
    fn get_date() -> (Year, i32) {
        (Year::Year2022, 1)
    }

    fn part_1(input: &str) -> String {
        let elves_calories = Self::generate_elves_calories(input);

        elves_calories.first().unwrap().to_owned().to_string()
    }

    fn part_2(input: &str) -> String {
        let elves_calories = Self::generate_elves_calories(input);

        (elves_calories[0..3].iter().sum::<i32>()).to_string()
    }
}