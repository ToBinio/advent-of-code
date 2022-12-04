use crate::advent_of_code::day::Day;
use crate::advent_of_code::input::sum;

pub struct Day2;

impl Day2 {
    pub fn get_sizes(sizes: &str) -> (i32, i32, i32) {
        let sizes: Vec<i32> = sizes.split('x').into_iter().map(|size| size.parse::<i32>().unwrap()).collect();
        (*sizes.first().unwrap(), *sizes.get(1).unwrap(), *sizes.get(2).unwrap())
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

    pub fn get_needed_ribbon(sizes: &str) -> i32 {
        let sizes = Day2::get_sizes(sizes);

        let mut sizes_vec = vec![sizes.0, sizes.1, sizes.2];

        sizes_vec.sort();

        let mut sum = 0;

        sum += sizes_vec.first().unwrap() * 2;
        sum += sizes_vec.get(1).unwrap() * 2;

        sum += sizes.0 * sizes.1 * sizes.2;

        sum
    }
}

impl Day for Day2 {
    fn get_path() -> String {
        "resources/year_2015/day2_data.txt".to_string()
    }

    fn part_1(input: &str) -> String {
        sum(input, Day2::get_needed_wrapping_paper).to_string()
    }

    fn part_2(input: &str) -> String {
        sum(input, Day2::get_needed_ribbon).to_string()
    }
}