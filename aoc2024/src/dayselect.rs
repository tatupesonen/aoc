use aoc2024::Solution;

pub fn select_day(day: usize) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(aoc2024::days::day01::Problem)),
        2 => Some(Box::new(aoc2024::days::day02::Problem)),
        3 => Some(Box::new(aoc2024::days::day03::Problem)),
        _ => None,
    }
}
