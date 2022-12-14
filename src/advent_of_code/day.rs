use std::{fs, panic};

pub trait Day {
    fn get_date() -> (Year, i32);

    fn part_1(input: &str) -> String;
    fn part_2(input: &str) -> String;

    fn get_test_input() -> String {
        let date = Self::get_date();
        fs::read_to_string(format!("resources/year{}/day{}_test.txt", date.0.value(), date.1)).unwrap_or_else(|_| panic!("Test.txt of Year{}-Day{} not Found", date.0.value(), date.1))
    }

    fn get_input() -> String {
        let date = Self::get_date();
        fs::read_to_string(format!("resources/year{}/day{}_data.txt", date.0.value(), date.1)).unwrap_or_else(|_| panic!("Data.txt of Year{}-Day{} not Found", date.0.value(), date.1))
    }

    fn run() {
        let date = Self::get_date();
        let input = Self::get_input();

        println!("Year{} - Day{}", date.0.value(), date.1);
        println!("Part 1");
        println!("{}", Self::part_1(input.as_str()));

        let _ = panic::catch_unwind(|| {
            let part2 = Self::part_2(input.as_str());

            println!("\nPart 2");
            println!("{}", part2);
        });
    }
}

pub enum Year {
    Year2015,
    Year2016,
    Year2022,
}

impl Year {
    pub fn value(&self) -> i32 {
        match self {
            Year::Year2015 => { 2015 }
            Year::Year2016 => { 2016 }
            Year::Year2022 => { 2022 }
        }
    }
}