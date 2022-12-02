mod days;

use days::day2;


const INPUT: &str = include_str!("./input.txt");

pub trait Solution {
    fn part_one(input: &str) -> String;
    fn part_two(input: &str) -> String;
}

fn main() {
    println!("Day one solutions:");
    // println!("{}", day1::Problem::part_one(INPUT));
    // println!("{}", day1::Problem::part_two(INPUT));

    println!("Day two solutions:");
    println!("{}", day2::Problem::part_one(INPUT));
    println!("{}", day2::Problem::part_two(INPUT));
}
