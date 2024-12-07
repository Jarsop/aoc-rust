mod aoc2024;
mod utils;

use chrono::{Datelike, Utc};
use clap::Parser;
use utils::solver::Solver;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'd', long = "day")]
    day: Option<u32>,
    #[arg(short = 'y', long = "year")]
    year: Option<u32>,
    #[arg(short = 'r', long = "refetch")]
    refetch: bool,
}

fn main() {
    let args = Args::parse();
    let now = Utc::now();
    let day = args.day.unwrap_or(now.day());
    let month = now.month();
    let year = args
        .year
        .unwrap_or(now.year().try_into().expect("year should be u32"));

    if month != 12 && (args.day.is_none() || args.year.is_none()) {
        eprintln!("It's not December yet, please specify a day and year with -d and -y.");
        std::process::exit(1);
    }

    let _ = match year {
        2024 => match day {
            1 => aoc2024::day01::Solver.solve(args.refetch),
            _ => {
                eprintln!("✘ No solver available for day {day} of Advent of Code {year}");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("✘ No solver available for day {day} of Advent of Code {year}");
            std::process::exit(1);
        }
    };
}
