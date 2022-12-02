use crate::advent_of_code::day::Day;

pub struct Day1 {}

impl Day1 {
    pub fn generate_elves_calories(input: &str) -> Vec<i32> {
        let mut elves_calories = Vec::new();

        let mut sum = 0;

        for line in input.lines() {
            if line == "" {
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

    pub fn compute_part_1(input: &str) -> i32 {
        let elves_calories = Self::generate_elves_calories(input);

        elves_calories.get(0).unwrap().to_owned()
    }

    pub fn compute_part_2(input: &str) -> i32 {
        let elves_calories = Self::generate_elves_calories(input);

        elves_calories[0..3].iter().sum()
    }
}

impl Day for Day1 {
    fn part_1() -> String {
        Day1::compute_part_1(include_str!("../../resources/year_2022/day1/data.txt")).to_string()
    }

    fn part_2() -> Option<String> {
        Some(Day1::compute_part_2(include_str!("../../resources/year_2022/day1/data.txt")).to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::year_2022::day_1::Day1;

    #[test]
    fn part_1_test() {
        assert_eq!(Day1::compute_part_1(include_str!("../../resources/year_2022/day1/test.txt")), 24000)
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Day1::compute_part_2(include_str!("../../resources/year_2022/day1/test.txt")), 45000)
    }
}