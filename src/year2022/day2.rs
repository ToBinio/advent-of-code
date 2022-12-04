use crate::advent_of_code::day::Day;

pub struct Day2;

impl Day2 {
    fn get_score(action: &str) -> i64 {
        match action {
            "X" | "A" => { 1 }
            "Y" | "B" => { 2 }
            _ => { 3 }
        }
    }

    pub fn get_what_to_choose(action: (&str, &str)) -> String {
        let mut goal = Self::get_score(action.0) - 1;

        match action.1 {
            "X" => {
                goal -= 1;
            }
            "Z" => {
                goal += 1;
            }
            _ => {}
        };

        goal = (goal + 3) % 3;

        match goal {
            0 => {
                "A".to_string()
            }
            1 => {
                "B".to_string()
            }
            _ => {
                "C".to_string()
            }
        }
    }

    pub fn get_result(actions: (&str, &str)) -> i64 {
        let actions = (Self::get_score(actions.0) - 1, Self::get_score(actions.1) - 1);

        if actions.0 == actions.1 { return 3; }
        if (actions.0 + 1) % 3 == (actions.1) % 3 { return 6; }

        0
    }
}

impl Day for Day2 {
    fn get_path() -> String {
        "resources/year2022/day2/data.txt".to_string()
    }

    fn part_1(input: &str) -> String {
        let mut sum = 0;

        for line in input.lines() {
            let mut split = line.split(' ');
            let split = (split.next().unwrap(), split.next().unwrap());

            sum += Self::get_score(split.1);
            sum += Self::get_result(split);
        }

        sum.to_string()
    }

    fn part_2(input: &str) -> String {
        let mut sum = 0;

        for line in input.lines() {
            let mut split = line.split(' ');
            let mut split = (split.next().unwrap(), split.next().unwrap());

            let own_chosen = Self::get_what_to_choose(split);

            split.1 = own_chosen.as_str();

            sum += Self::get_score(split.1);
            sum += Self::get_result(split);
        }

        sum.to_string()
    }
}