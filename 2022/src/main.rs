#![feature(iter_array_chunks)]

use clap::{arg, Parser};
use owo_colors::{OwoColorize, Stream::Stdout};
mod days;

pub trait Solution {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

fn select_day(day: usize) -> Option<Box<dyn Solution>> {
    daymport::dir!("src/days")
}

fn run_all_days() {
    let days = (1..25).filter_map(select_day);
    for (day_num, sol) in days.into_iter().enumerate() {
        let day_num = day_num + 1;
        run_day(sol, day_num);
    }
}

fn get_input(day_num: usize) -> String {
    std::fs::read_to_string(format!("./inputs/{}/input.txt", day_num))
        .expect("Input file doesn't exist.")
}

fn run_day(day: Box<dyn Solution>, day_num: usize) {
    println!("****** Solutions for day {day_num} ******");
    let input = get_input(day_num);
    let part1 = day.part_one(&input);
    println!("Part 1: {}", part1.if_supports_color(Stdout, |text| text.bright_blue()));
    let part2 = day.part_two(&input);
    println!("Part 2: {}", part2.if_supports_color(Stdout, |text| text.bright_blue()));
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long)]
    day: Option<usize>,
}

fn main() {
    let args = Args::parse();
    if let Some(day) = args.day {
        let solution = select_day(day);
        match solution {
            Some(sol) => run_day(sol, day),
            None => {
                eprintln!("{}", "No solution for day found for given day.".if_supports_color(Stdout, |text| text.bright_red()));
                std::process::exit(1);
            }
        }
    } else {
        run_all_days();
    }
}
