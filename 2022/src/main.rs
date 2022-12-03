#![feature(iter_array_chunks)]

use clap::{arg, Parser};
mod days;

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

fn select_day(day: usize) -> (usize, Box<dyn Solution>) {
    let solution: Box<dyn Solution> = match day {
        1 => Box::new(days::day1::Problem),
        2 => Box::new(days::day2::Problem),
        3 => Box::new(days::day3::Problem),
        _ => panic!("no such day"),
    };

    (day, solution)
}

fn run_day(input: &str, day: Box<dyn Solution>) -> () {
    let part1 = day.part_one(input);
    println!("Part 1: {}", part1);
    let part2 = day.part_two(input);
    println!("Part 2: {}", part2);
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long)]
    day: usize,
}

fn main() {
    let args = Args::parse();
    let (day, solution) = select_day(args.day);
    // Get input
    let input = std::fs::read_to_string(format!("./inputs/{}/input.txt", day))
        .expect("Input file doesn't exist.");
    println!("****** Solutions for day {} ******", day);
    run_day(&input, solution);
}
