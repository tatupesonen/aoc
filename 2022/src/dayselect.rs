use aoc2022::Solution;

pub fn select_day(day: usize) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(aoc2022::days::day01::Problem)),
        2 => Some(Box::new(aoc2022::days::day02::Problem)),
        3 => Some(Box::new(aoc2022::days::day03::Problem)),
        4 => Some(Box::new(aoc2022::days::day04::Problem)),
        5 => Some(Box::new(aoc2022::days::day05::Problem)),
        6 => Some(Box::new(aoc2022::days::day06::Problem)),
        7 => Some(Box::new(aoc2022::days::day07::Problem)),
        8 => Some(Box::new(aoc2022::days::day08::Problem)),
        9 => Some(Box::new(aoc2022::days::day09::Problem)),
        10 => Some(Box::new(aoc2022::days::day10::Problem)),
        11 => Some(Box::new(aoc2022::days::day11::Problem)),
        12 => Some(Box::new(aoc2022::days::day12::Problem)),
        _ => None,
    }
}
