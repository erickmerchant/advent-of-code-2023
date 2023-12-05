pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use clap::{Parser, ValueEnum};
use std::io::{self, BufRead};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short, value_enum)]
    day: Days,
    #[arg(long, short, value_enum)]
    part: Parts,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Days {
    #[clap(name = "1")]
    Day1,

    #[clap(name = "2")]
    Day2,

    #[clap(name = "3")]
    Day3,

    #[clap(name = "4")]
    Day4,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Parts {
    #[clap(name = "1")]
    Part1,

    #[clap(name = "2")]
    Part2,
}

fn main() {
    let args = Args::parse();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let input = handle.lines().flatten().collect();
    let output = match (args.day, args.part) {
        (Days::Day1, Parts::Part1) => day1::part1(input),
        (Days::Day1, Parts::Part2) => day1::part2(input),
        (Days::Day2, Parts::Part1) => day2::part1(input),
        (Days::Day2, Parts::Part2) => day2::part2(input),
        (Days::Day3, Parts::Part1) => day3::part1(input),
        (Days::Day3, Parts::Part2) => day3::part2(input),
        (Days::Day4, Parts::Part1) => day4::part1(input),
        (Days::Day4, Parts::Part2) => day4::part2(input),
    };

    println!("{output}");
}
