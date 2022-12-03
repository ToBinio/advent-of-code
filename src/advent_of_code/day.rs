use std::fs;

pub trait Day {
    fn get_path() -> String;

    fn part_1(input: &str) -> String;
    fn part_2(input: &str) -> Option<String>;

    fn run() {
        let input = fs::read_to_string(Self::get_path()).unwrap_or_else(|_| panic!("File {} not Found", Self::get_path()));

        println!("Part 1");
        println!("{}", Self::part_1(input.as_str()));

        let part2 = Self::part_2(input.as_str());

        if part2.is_none() {
            return;
        }

        println!("\nPart 2");
        println!("{}", part2.unwrap());
    }
}