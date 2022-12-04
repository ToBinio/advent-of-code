use advent_of_code::year_2022::day4::Day4;

#[test]
fn range_contains_completely_test() {
    assert!(!Day4::range_contains_completely("2-4,6-8"));
    assert!(!Day4::range_contains_completely("2-3,4-5"));
    assert!(!Day4::range_contains_completely("5-7,7-9"));
    assert!(Day4::range_contains_completely("2-8,3-7"));
    assert!(Day4::range_contains_completely("6-6,4-6"));
    assert!(!Day4::range_contains_completely("2-6,4-8"));
}

#[test]
fn range_contains_partly_test() {
    assert!(!Day4::range_contains_partly("2-4,6-8"));
    assert!(!Day4::range_contains_partly("2-3,4-5"));
    assert!(Day4::range_contains_partly("5-7,7-9"));
    assert!(Day4::range_contains_partly("2-8,3-7"));
    assert!(Day4::range_contains_partly("6-6,4-6"));
    assert!(Day4::range_contains_partly("2-6,4-8"));
}