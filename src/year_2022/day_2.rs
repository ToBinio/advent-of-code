use crate::advent_of_code::day::Day;

pub struct Day2 {}

impl Day2 {
    pub fn compute_part_1(input: &str) -> i64 {
        let mut sum = 0;

        for line in input.lines() {
            let mut split = line.split(" ");
            let split = (split.next().unwrap(), split.next().unwrap());

            sum += Self::get_score(split.1);
            sum += Self::get_result(split);
        }

        sum
    }

    pub fn compute_part_2(input: &str) -> i64 {
        let mut sum = 0;

        for line in input.lines() {
            let mut split = line.split(" ");
            let mut split = (split.next().unwrap(), split.next().unwrap());

            let own_chosen = Self::get_what_to_choose(split);

            split.1 = own_chosen.as_str();

            sum += Self::get_score(split.1);
            sum += Self::get_result(split);
        }

        sum
    }

    fn get_score(action: &str) -> i64 {
        match action {
            "X" | "A" => { 1 }
            "Y" | "B" => { 2 }
            _ => { 3 }
        }
    }

    fn get_what_to_choose(action: (&str, &str)) -> String {
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

        return match goal {
            0 => {
                "A".to_string()
            }
            1 => {
                "B".to_string()
            }
            _ => {
                "C".to_string()
            }
        };
    }

    fn get_result(actions: (&str, &str)) -> i64 {
        let actions = (Self::get_score(actions.0) - 1, Self::get_score(actions.1) - 1);

        if actions.0 == actions.1 { return 3; }
        if (actions.0 + 1) % 3 == (actions.1) % 3 { return 6; }

        0
    }
}

impl Day for Day2 {
    fn part_1() -> String {
        Day2::compute_part_1(include_str!("../../resources/year_2022/day2/data.txt")).to_string()
    }

    fn part_2() -> Option<String> {
        Some(Day2::compute_part_2(include_str!("../../resources/year_2022/day2/data.txt")).to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::year_2022::day_2::Day2;

    #[test]
    fn part_1_test() {
        assert_eq!(Day2::compute_part_1(include_str!("../../resources/year_2022/day2/test.txt")), 15);
    }

    #[test]
    fn get_result() {
        assert_eq!(Day2::get_result(("A", "X")), 3);
        assert_eq!(Day2::get_result(("B", "Y")), 3);
        assert_eq!(Day2::get_result(("C", "Z")), 3);

        assert_eq!(Day2::get_result(("A", "Z")), 0);
        assert_eq!(Day2::get_result(("B", "X")), 0);
        assert_eq!(Day2::get_result(("C", "Y")), 0);

        assert_eq!(Day2::get_result(("A", "Y")), 6);
        assert_eq!(Day2::get_result(("B", "Z")), 6);
        assert_eq!(Day2::get_result(("C", "X")), 6);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Day2::compute_part_2(include_str!("../../resources/year_2022/day2/test.txt")), 12);
    }

    #[test]
    fn get_what_to_choose() {
        assert_eq!(Day2::get_what_to_choose(("B", "Y")), "B");
        assert_eq!(Day2::get_what_to_choose(("A", "X")), "C");
        assert_eq!(Day2::get_what_to_choose(("C", "Z")), "A");

        assert_eq!(Day2::get_what_to_choose(("C", "Y")), "C");
        assert_eq!(Day2::get_what_to_choose(("B", "X")), "A");
        assert_eq!(Day2::get_what_to_choose(("A", "Z")), "B");

        assert_eq!(Day2::get_what_to_choose(("A", "Y")), "A");
        assert_eq!(Day2::get_what_to_choose(("C", "X")), "B");
        assert_eq!(Day2::get_what_to_choose(("B", "Z")), "C");
    }
}