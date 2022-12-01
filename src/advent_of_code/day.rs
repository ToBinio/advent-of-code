pub trait Day {
    fn part_1(&mut self) -> String;
    fn part_2(&mut self) -> String;

    fn run(&mut self) {
        println!("Part 1");
        println!("{:?}", self.part_1());

        println!("Part 2");
        println!("{:?}", self.part_2());
    }
}