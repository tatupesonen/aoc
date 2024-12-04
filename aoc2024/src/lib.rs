#![feature(iter_array_chunks)]
#![feature(iter_intersperse)]

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn get_input(day_num: usize, test: bool) -> String {
    if !test {
        std::fs::read_to_string(format!("./inputs/{day_num:02}/input.txt"))
            .expect("Input file doesn't exist.")
    } else {
        std::fs::read_to_string(format!("./inputs/{day_num:02}/test-input.txt"))
            .expect("Input file doesn't exist.")
    }
}

#[macro_export]
macro_rules! day {
    ($day:literal, 1 => $part1_impl:block, 2 => $part2_impl:block) => {
        pub struct Problem;
        impl Solution for Problem {
            fn part_one(&self, input: &str) -> String {
                $part1_impl
            }

            fn part_two(&self, input: &str) -> String {
                $part2_impl
            }
        }
    };
}

#[macro_export]
macro_rules! test_input {
    ($day:literal) => {
        include_str!(concat!("../../inputs/", $day, "/test-input.txt"))
    };
}

#[macro_export]
macro_rules! input {
    ($day:literal) => {
        include_str!(concat!("../../inputs/", $day, "/input.txt"))
    };
}

pub mod days;
pub use days::*;
