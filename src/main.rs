pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use clap::Parser;
use core::panic;
use std::io::{self, BufRead};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short)]
    day: u8,
    #[arg(long, short)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let input = handle.lines().flatten().collect();
    let output = match (args.day, args.part) {
        (1, 1) => day1::part1(input),
        (1, 2) => day1::part2(input),
        (2, 1) => day2::part1(input),
        (2, 2) => day2::part2(input),
        (3, 1) => day3::part1(input),
        (3, 2) => day3::part2(input),
        (4, 1) => day4::part1(input),
        (4, 2) => day4::part2(input),
        (5, 1) => day5::part1(input),
        (5, 2) => day5::part2(input),
        (6, 1) => day6::part1(input),
        (6, 2) => day6::part2(input),
        (7, 1) => day7::part1(input),
        (7, 2) => day7::part2(input),
        (8, 1) => day8::part1(input),
        (8, 2) => day8::part2(input),
        (9, 1) => day9::part1(input),
        (9, 2) => day9::part2(input),
        (10, 1) => day10::part1(input),
        (10, 2) => day10::part2(input),
        (11, 1) => day11::part1(input),
        (11, 2) => day11::part2(input),
        (12, 1) => day12::part1(input),
        (12, 2) => day12::part2(input),
        (13, 1) => day13::part1(input),
        (13, 2) => day13::part2(input),
        (14, 1) => day14::part1(input),
        (14, 2) => day14::part2(input),
        _ => panic!("Incomplete day or part"),
    };

    println!("{output}");
}
