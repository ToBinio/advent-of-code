use std::{fs, panic};

pub trait Day {
    fn get_path() -> String;

    fn part_1(input: &str) -> String;
    fn part_2(input: &str) -> String;

    fn run() {
        let input = fs::read_to_string(Self::get_path()).unwrap_or_else(|_| panic!("File {} not Found", Self::get_path()));

        println!("Part 1");
        println!("{}", Self::part_1(input.as_str()));

        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));

        let _ = panic::catch_unwind(|| {
            let part2 = Self::part_2(input.as_str());

            println!("\nPart 2");
            println!("{}", part2);
        });

        panic::set_hook(prev_hook);
    }
}