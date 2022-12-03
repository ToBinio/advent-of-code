use advent_of_code::year_2015::day_2::Day2;

#[test]
fn get_needed_wrapping_paper_test() {
    assert_eq!(Day2::get_needed_wrapping_paper("2x3x4"), 58);
    assert_eq!(Day2::get_needed_wrapping_paper("1x1x10"), 43);
}

#[test]
fn get_needed_wrapping_ribbon() {
    assert_eq!(Day2::get_needed_ribbon("2x3x4"), 34);
    assert_eq!(Day2::get_needed_ribbon("1x1x10"), 14);
}