#![feature(iter_array_chunks)]
#![feature(iter_intersperse)]

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn get_input(day_num: usize, test: bool) -> String {
    if !test {
        std::fs::read_to_string(format!("./inputs/{}/input.txt", day_num))
            .expect("Input file doesn't exist.")
    } else {
        std::fs::read_to_string(format!("./inputs/{}/test-input.txt", day_num))
            .expect("Input file doesn't exist.")
    }
}

pub mod days;
pub use days::*;
