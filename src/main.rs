pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;

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
        (1, 1) => day01::part1(input),
        (1, 2) => day01::part2(input),
        (2, 1) => day02::part1(input),
        (2, 2) => day02::part2(input),
        (3, 1) => day03::part1(input),
        (3, 2) => day03::part2(input),
        (4, 1) => day04::part1(input),
        (4, 2) => day04::part2(input),
        (5, 1) => day05::part1(input),
        (5, 2) => day05::part2(input),
        (6, 1) => day06::part1(input),
        (6, 2) => day06::part2(input),
        (7, 1) => day07::part1(input),
        (7, 2) => day07::part2(input),
        (8, 1) => day08::part1(input),
        (8, 2) => day08::part2(input),
        (9, 1) => day09::part1(input),
        (9, 2) => day09::part2(input),
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
        (15, 1) => day15::part1(input),
        (15, 2) => day15::part2(input),
        (16, 1) => day16::part1(input),
        (16, 2) => day16::part2(input),
        (17, 1) => day17::part1(input),
        (17, 2) => day17::part2(input),
        (18, 1) => day18::part1(input),
        (18, 2) => day18::part2(input),
        (19, 1) => day19::part1(input),
        (19, 2) => day19::part2(input),
        (20, 1) => day20::part1(input),
        (20, 2) => day20::part2(input),
        (21, 1) => day21::part1(input),
        (21, 2) => day21::part2(input),
        _ => panic!("Incomplete day or part"),
    };

    println!("{output}");
}
