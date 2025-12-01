use aoc2025::Solution;

pub fn select_day(day: usize) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(aoc2025::days::day01::Problem)),
        _ => None,
    }
}
