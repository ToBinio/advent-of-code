use crate::advent_of_code::day::Day;

pub struct Day1 {}

impl Day1 {
    pub fn calc_floor(input: &str) -> i32 {
        let mut floor = 0;

        input.chars()
            .for_each(|action| {
                match action {
                    '(' => { floor += 1 }
                    ')' => { floor -= 1 }
                    _ => {}
                };
            });

        floor
    }

    pub fn calc_first_neg_floor(input: &str) -> i32 {
        let mut floor = 0;

        for (index, action) in input.chars().enumerate() {
            match action {
                '(' => { floor += 1 }
                ')' => { floor -= 1 }
                _ => {}
            };

            if floor < 0 {
                return (index + 1) as i32;
            }
        }

        floor
    }
}

impl Day for Day1 {
    fn part_1() -> String {
        Day1::calc_floor(include_str!("../../resources/year_2015/day1/data.txt")).to_string()
    }

    fn part_2() -> Option<String> {
        Some(Day1::calc_first_neg_floor(include_str!("../../resources/year_2015/day1/data.txt")).to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::year_2015::day_1::Day1;

    #[test]
    fn part_1_test() {
        assert_eq!(Day1::calc_floor("(())"), 0);
        assert_eq!(Day1::calc_floor("()()"), 0);

        assert_eq!(Day1::calc_floor("((("), 3);
        assert_eq!(Day1::calc_floor("(()(()("), 3);
        assert_eq!(Day1::calc_floor("))((((("), 3);

        assert_eq!(Day1::calc_floor("())"), -1);
        assert_eq!(Day1::calc_floor("))("), -1);

        assert_eq!(Day1::calc_floor(")))"), -3);
        assert_eq!(Day1::calc_floor(")())())"), -3);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(Day1::calc_first_neg_floor(")"), 1);
        assert_eq!(Day1::calc_first_neg_floor("()())"), 5);
    }
}