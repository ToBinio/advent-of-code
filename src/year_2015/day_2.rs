use crate::advent_of_code::day::Day;

pub struct Day2 {}

impl Day2 {
    pub fn get_sizes(sizes: &str) -> (i32, i32, i32) {
        let sizes: Vec<i32> = sizes.split("x").into_iter().map(|size| size.parse::<i32>().unwrap()).collect();
        (*sizes.get(0).unwrap(), *sizes.get(1).unwrap(), *sizes.get(2).unwrap())
    }

    pub fn get_needed_wrapping_paper(sizes: &str) -> i32 {
        let sizes = Day2::get_sizes(sizes);

        let site1 = sizes.0 * sizes.1;
        let site2 = sizes.1 * sizes.2;
        let site3 = sizes.2 * sizes.0;

        let mut sum = site1.min(site2).min(site3);

        sum += site1 * 2;
        sum += site2 * 2;
        sum += site3 * 2;

        sum
    }

    pub fn get_needed_Ribbon(sizes: &str) -> i32 {
        let sizes = Day2::get_sizes(sizes);

        let mut sizes_Vec = Vec::new();

        sizes_Vec.push(sizes.0);
        sizes_Vec.push(sizes.1);
        sizes_Vec.push(sizes.2);

        sizes_Vec.sort();

        let mut sum = 0;

        sum += sizes_Vec.get(0).unwrap() * 2;
        sum += sizes_Vec.get(1).unwrap() * 2;

        sum += sizes.0 * sizes.1 * sizes.2;

        sum
    }
}

impl Day for Day2 {
    fn part_1() -> String {
        let input = include_str!("../../resources/year_2015/day2/data.txt");

        let sum: i32 = input.lines().map(|line| Day2::get_needed_wrapping_paper(line)).sum();

        sum.to_string()
    }

    fn part_2() -> Option<String> {
        let input = include_str!("../../resources/year_2015/day2/data.txt");

        let sum: i32 = input.lines().map(|line| Day2::get_needed_Ribbon(line)).sum();

        Some(sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::year_2015::day_2::Day2;

    #[test]
    fn get_needed_wrapping_paper_test() {
        assert_eq!(Day2::get_needed_wrapping_paper("2x3x4"), 58);
        assert_eq!(Day2::get_needed_wrapping_paper("1x1x10"), 43);
    }

    #[test]
    fn get_needed_wrapping_ribbon() {
        assert_eq!(Day2::get_needed_Ribbon("2x3x4"), 34);
        assert_eq!(Day2::get_needed_Ribbon("1x1x10"), 14);
    }
}