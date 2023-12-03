use advent::{day3::*, read_stdin};

fn main() {
    let mut collection = Vec::<u32>::new();
    let (symbols, numbers) = parse_input(read_stdin().flatten().collect());

    for (location, symbol) in symbols {
        if symbol == '*' {
            let mut part_numbers = Vec::<u32>::new();

            for number in &numbers {
                if number.field.contains(&location) {
                    part_numbers.push(number.value);
                }
            }

            if part_numbers.len() == 2 {
                collection.push(part_numbers[0] * part_numbers[1]);
            }
        }
    }

    let result = &collection.iter().sum::<u32>();

    println!("{result:?}");
}
