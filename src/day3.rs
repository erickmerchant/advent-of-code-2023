use std::collections::{HashMap, HashSet};

type Coords = (usize, usize);
type CoordsSet = HashSet<Coords>;
type SymbolMap = HashMap<Coords, char>;
type NumberList = Vec<Number>;
type Output = (SymbolMap, NumberList);

#[derive(Default, PartialEq, Eq, Debug)]
struct Number {
    value: usize,
    field: CoordsSet,
}

pub fn part1(input: Vec<String>) -> usize {
    let mut collection = Vec::<usize>::new();
    let (symbols, numbers) = parse_input(input);
    let symbol_locations: CoordsSet = symbols.keys().cloned().collect();

    for number in numbers {
        if number.field.intersection(&symbol_locations).count() > 0 {
            collection.push(number.value);
        }
    }

    let result = &collection.iter().sum::<usize>();

    *result
}

pub fn part2(input: Vec<String>) -> usize {
    let mut collection = Vec::<usize>::new();
    let (symbols, numbers) = parse_input(input);

    for (location, symbol) in symbols {
        if symbol == '*' {
            let mut part_numbers = Vec::<usize>::new();

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

    let result = &collection.iter().sum::<usize>();

    *result
}

fn parse_input(input: Vec<String>) -> Output {
    let mut symbols = SymbolMap::new();
    let mut numbers = NumberList::new();
    let mut current: Number = Default::default();

    for (row, line) in (0..).zip(input) {
        for (column, char) in (0..).zip(line.chars()) {
            if char.is_ascii_digit() {
                current.value = (current.value * 10)
                    + char
                        .to_string()
                        .parse::<usize>()
                        .expect("should be a number");

                if row > 0 && column > 0 {
                    current.field.insert((row - 1, column - 1));
                }

                if row > 0 {
                    current.field.insert((row - 1, column));
                    current.field.insert((row - 1, column + 1));
                }

                if column > 0 {
                    current.field.insert((row, column - 1));
                }

                current.field.insert((row, column + 1));

                if column > 0 {
                    current.field.insert((row + 1, column - 1));
                }

                current.field.insert((row + 1, column));
                current.field.insert((row + 1, column + 1));
            } else {
                if char != '.' {
                    symbols.insert((row, column), char);
                }

                if current.value > 0 {
                    numbers.push(current);
                    current = Default::default();
                }
            }
        }
    }

    (symbols, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture = get_fixture();

        assert_eq!(part1(fixture), 4361);
    }

    #[test]
    fn test_part2() {
        let fixture = get_fixture();

        assert_eq!(part2(fixture), 467835);
    }
}
