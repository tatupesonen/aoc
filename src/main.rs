#![feature(iter_array_chunks)]
mod days;


use days::day1;
use days::day2;
use days::day3;


const DAY_3: &str = include_str!("../inputs/3/input.txt");

pub trait Solution {
    fn part_one(input: &str) -> String;
    fn part_two(input: &str) -> String;
}

fn main() {
    println!("Day one solutions:");
    // println!("{}", day1::Problem::part_one(INPUT));
    // println!("{}", day1::Problem::part_two(INPUT));

    println!("Day two solutions:");
    // println!("{}", day2::Problem::part_one(INPUT));
    // println!("{}", day2::Problem::part_two(INPUT));

    println!("Day 3 solutions:");
    println!("{}", day3::Problem::part_one(DAY_3));
    println!("{}", day3::Problem::part_two(DAY_3));
}
