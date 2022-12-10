use aoc2022::Solution;

pub fn select_day(day: usize) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(aoc2022::days::day1::Problem)),
        2 => Some(Box::new(aoc2022::days::day2::Problem)),
        3 => Some(Box::new(aoc2022::days::day3::Problem)),
        4 => Some(Box::new(aoc2022::days::day4::Problem)),
        5 => Some(Box::new(aoc2022::days::day5::Problem)),
        6 => Some(Box::new(aoc2022::days::day6::Problem)),
        7 => Some(Box::new(aoc2022::days::day7::Problem)),
        8 => Some(Box::new(aoc2022::days::day8::Problem)),
        9 => Some(Box::new(aoc2022::days::day9::Problem)),
        10 => Some(Box::new(aoc2022::days::day10::Problem)),
        _ => None,
    }
}
