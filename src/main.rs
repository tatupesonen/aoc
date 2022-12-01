mod days;
use days::day1;
const INPUT: &str = include_str!("./input.txt");

pub trait Problem {
    fn part_one(input: &str) -> ();
    fn part_two(input: &str) -> ();
}

fn main() {
    println!("Day one solutions:");
    day1::Solution::part_one(INPUT);
    day1::Solution::part_two(INPUT);
}