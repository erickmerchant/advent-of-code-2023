use advent::{day1::*, read_stdin};

fn main() {
    let mut collection = Vec::<u32>::new();

    for line in read_stdin() {
        collection.push(parse_numbers_plus(line.expect("should be a line")));
    }

    let result = &collection.iter().sum::<u32>();

    println!("{result:?}");
}
