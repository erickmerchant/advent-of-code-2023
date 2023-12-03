use advent::{day3::*, read_stdin};

fn main() {
    let mut collection = Vec::<u32>::new();
    let (symbols, numbers) = parse_input(read_stdin().flatten().collect());

    let symbol_locations: CoordsSet = symbols.keys().cloned().collect();

    for number in numbers {
        if number.field.intersection(&symbol_locations).count() > 0 {
            collection.push(number.value);
        }
    }

    let result = &collection.iter().sum::<u32>();

    println!("{result:?}");
}
