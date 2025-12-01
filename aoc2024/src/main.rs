#![feature(iter_array_chunks)]
#![feature(iter_intersperse)]

pub mod dayselect;

use aoc2024::*;
use clap::{Parser, Subcommand};
use owo_colors::{OwoColorize, Stream::Stdout};

fn run_all_days(test: bool) -> miette::Result<()> {
    let days = (1..25).filter_map(dayselect::select_day);
    for (day_num, sol) in days.into_iter().enumerate() {
        let day_num = day_num + 1;
        run_day(sol, day_num, test)?;
    }

    Ok(())
}

pub fn run_day(day: Box<dyn Solution>, day_num: usize, test: bool) -> miette::Result<()> {
    println!("****** Solutions for day {day_num} ******");
    let input = get_input(day_num, test);
    let part1 = day.part_one(&input);
    match part1 {
        Ok(res) => println!(
            "Part 1: {}",
            res.if_supports_color(Stdout, |text| text.bright_blue())
        ),
        Err(e) => eprintln!("Day {} part 1 failed running:\n{:?}", day_num, e)
    }
    let part2 = day.part_two(&input);
    match part2 {
        Ok(res) => println!(
            "Part 2: {}",
            res.if_supports_color(Stdout, |text| text.bright_blue())
        ),
        Err(e) => eprintln!("Day {} part 2 failed running:\n{}", day_num, e)
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        /// Day to run
        #[arg(short, long)]
        day: Option<usize>,
        /// Flag to enable test input
        #[arg(short, long, default_value_t = false)]
        test: bool,
    },
    Template {
        /// Day to generate template for
        #[arg(short, long)]
        day: usize,
    },
}

fn main() -> miette::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Run { day, test } => {
            if let Some(day) = day {
                let solution = dayselect::select_day(day);
                match solution {
                    Some(sol) => run_day(sol, day, test)?,
                    None => {
                        eprintln!(
                            "{}",
                            "No solution found for given day."
                                .if_supports_color(Stdout, |text| text.bright_red())
                        );
                        std::process::exit(1);
                    }
                }
            } else {
                run_all_days(test)?;
            }
        }
        Command::Template { day: _ } => {
            todo!();
        }
    }
    Ok(())
}

pub fn template_file() {}
