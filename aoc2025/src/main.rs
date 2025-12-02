pub mod days;
pub mod util;

use std::{fs, path::Path};
const YEAR: u16 = 2024;

use aoc2025::{get_input, Solution};
use clap::{arg, Parser, Subcommand};
use miette::{IntoDiagnostic, Result, WrapErr};
use owo_colors::{OwoColorize, Stream::Stdout};
use tera::{Context, Tera};

fn run_all_days(test: bool) -> Result<()> {
    let days = (1..25).filter_map(days::select_day);
    for sol in days.into_iter() {
        let _ = run_day(sol, test);
    }

    Ok(())
}

pub fn run_day(day: Box<dyn Solution>, test: bool) -> Result<()> {
    let day_num = day.day();
    let out = format!("****** Solutions for day {day_num} ******");
    let out = out.bright_red();
    println!("{out}");
    let input = get_input(day_num, test);
    if input.is_none() {
        println!("Day {day_num:02} not implemented...");
        return Ok(());
    }
    let input = input.unwrap();

    match day.part_one(&input) {
        Ok(res) => println!(
            "Part 1: {}",
            res.if_supports_color(Stdout, |text| text.bright_green())
        ),
        Err(e) => eprintln!("Day {day_num} part 1 failed:\n{e:?}"),
    }

    match day.part_two(&input) {
        Ok(res) => println!(
            "Part 2: {}",
            res.if_supports_color(Stdout, |text| text.bright_green())
        ),
        Err(e) => eprintln!("Day {day_num} part 2 failed:\n{e}"),
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
        #[arg(short, long)]
        day: Option<usize>,
        #[arg(short, long, default_value_t = false)]
        test: bool,
    },
    Template {
        #[arg(short, long)]
        day: usize,
    },
}

fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let args = Args::parse();
    match args.command {
        Command::Run { day, test } => {
            if let Some(day) = day {
                let solution = days::select_day(day);
                match solution {
                    Some(sol) => run_day(sol, test)?,
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

        Command::Template { day } => {
            let day_padded = format!("{:02}", day);

            let mut tera = Tera::default();
            tera.add_template_file("template/day.rs.tera", Some("day"))
                .into_diagnostic()
                .wrap_err("Failed to load template/day.rs.tera")?;

            let mut context = Context::new();
            context.insert("day", &day);
            context.insert("day_padded", &day_padded);

            let rendered = tera
                .render("day", &context)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed rendering template for day {day_padded}"))?;

            let out_path = format!("src/days/day{day_padded}.rs");
            fs::write(&out_path, rendered)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed writing generated day file {out_path}"))?;

            println!("Created {out_path}");

            let session = std::env::var("SESSION")
                .into_diagnostic()
                .wrap_err("Missing SESSION env var. Set it to your session cookie.")?;

            let url = format!("https://adventofcode.com/{YEAR}/day/{day}/input");

            println!("Downloading input for day {day_padded}...");

            let mut resp = ureq::get(&url)
                .header("Cookie", &format!("session={session}"))
                .call()
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed to request AoC input for day {day}"))?;

            if resp.status() != 200 {
                return Err(miette::miette!(
                    "AoC returned error {} for day {}",
                    resp.status(),
                    day
                ));
            }

            let input_data = resp
                .body_mut()
                .read_to_string()
                .into_diagnostic()
                .wrap_err("Failed to read AoC input response body")?;

            fs::create_dir_all(format!("inputs/{day_padded}"))
                .into_diagnostic()
                .wrap_err("Failed to create inputs/ directory")?;

            let input_path = format!("inputs/{day_padded}/input.txt");
            fs::write(&input_path, input_data)
                .into_diagnostic()
                .wrap_err_with(|| format!("Failed writing {}", input_path))?;

            let test_path = format!("inputs/{day_padded}/test-input.txt");
            if !Path::new(&test_path).exists() {
                fs::write(&test_path, "")
                    .into_diagnostic()
                    .wrap_err_with(|| format!("Failed creating {}", test_path))?;
            }

            println!("Downloaded input to {input_path}");
        }
    }
    Ok(())
}
