pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use std::io::{self, BufRead};

pub fn read_stdin() -> io::Lines<io::StdinLock<'static>> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    handle.lines()
}
