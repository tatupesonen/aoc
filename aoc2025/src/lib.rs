pub mod util;
pub trait Solution {
    fn part_one(&self, input: &str) -> miette::Result<String>;
    fn part_two(&self, input: &str) -> miette::Result<String>;
    fn day(&self) -> u16;
}

pub fn get_input(day_num: u16, test: bool) -> Option<String> {
    if !test {
        std::fs::read_to_string(format!("./inputs/{day_num:02}/input.txt")).ok()
    } else {
        std::fs::read_to_string(format!("./inputs/{day_num:02}/test-input.txt")).ok()
    }
}

#[macro_export]
macro_rules! day {
    ($day:literal, 1 => $part1_impl:expr, 2 => $part2_impl:expr) => {
        pub struct Problem;

        impl Solution for Problem {
            fn part_one(&self, s: &str) -> miette::Result<String> {
                $part1_impl
            }

            fn part_two(&self, s: &str) -> miette::Result<String> {
                $part2_impl
            }
        }
    };
}

#[macro_export]
macro_rules! test_input {
    ($day:tt) => {
        include_str!(concat!("../../inputs/", $day, "/test-input.txt"))
    };
}

#[macro_export]
macro_rules! input {
    ($day:literal) => {
        include_str!(concat!("../../inputs/", $day, "/input.txt"))
    };
}
