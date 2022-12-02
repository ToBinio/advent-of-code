pub trait Day {
    fn part_1() -> String;
    fn part_2() -> Option<String>;

    fn run() {
        println!("Part 1");
        println!("{:?}", Self::part_1());

        let part2 = Self::part_2();

        if part2.is_none() {
            return;
        }

        println!("Part 2");
        println!("{:?}", part2.unwrap());
    }
}