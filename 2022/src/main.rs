#![feature(iter_array_chunks)]
#![feature(iter_intersperse)]

pub mod utils;

use aoc2022::*;
use clap::{arg, Parser};
use owo_colors::{OwoColorize, Stream::Stdout};

fn select_day(day: usize) -> Option<Box<dyn Solution>> {
    daymport::dir!("src/days")
}

fn run_all_days(test: bool) {
    let days = (1..25).filter_map(select_day);
    for (day_num, sol) in days.into_iter().enumerate() {
        let day_num = day_num + 1;
        run_day(sol, day_num, test);
    }
}

pub fn run_day(day: Box<dyn Solution>, day_num: usize, test: bool) {
    println!("****** Solutions for day {day_num} ******");
    let input = get_input(day_num, test);
    let part1 = day.part_one(&input);
    println!(
        "Part 1: {}",
        part1.if_supports_color(Stdout, |text| text.bright_blue())
    );
    let part2 = day.part_two(&input);
    println!(
        "Part 2: {}",
        part2.if_supports_color(Stdout, |text| text.bright_blue())
    );
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long)]
    day: Option<usize>,

    /// Flag to enable test input
    #[arg(short, long, default_value_t = false)]
    test: bool,
}

fn main() {
    let args = Args::parse();
    if let Some(day) = args.day {
        let solution = select_day(day);
        match solution {
            Some(sol) => run_day(sol, day, args.test),
            None => {
                eprintln!(
                    "{}",
                    "No solution for day found for given day."
                        .if_supports_color(Stdout, |text| text.bright_red())
                );
                std::process::exit(1);
            }
        }
    } else {
        run_all_days(args.test);
    }
}
