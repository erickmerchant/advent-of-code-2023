use std::collections::{HashMap, HashSet};

pub fn part1(input: Vec<String>) -> u32 {
    let mut collection = Vec::<u32>::new();
    let (symbols, numbers) = parse_input(input);
    let symbol_locations: CoordsSet = symbols.keys().cloned().collect();

    for number in numbers {
        if number.field.intersection(&symbol_locations).count() > 0 {
            collection.push(number.value);
        }
    }

    let result = &collection.iter().sum::<u32>();

    *result
}

pub fn part2(input: Vec<String>) -> u32 {
    let mut collection = Vec::<u32>::new();
    let (symbols, numbers) = parse_input(input);

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

    *result
}

#[derive(Default, PartialEq, Eq, Debug)]
struct Number {
    value: u32,
    field: CoordsSet,
}

type Coords = (u32, u32);
type CoordsSet = HashSet<Coords>;
type SymbolMap = HashMap<Coords, char>;
type NumberList = Vec<Number>;
type Output = (SymbolMap, NumberList);

fn parse_input(input: Vec<String>) -> Output {
    let mut symbols = SymbolMap::new();
    let mut numbers = NumberList::new();
    let mut current: Number = Default::default();

    for (row, line) in (0_u32..).zip(input) {
        for (column, char) in (0_u32..).zip(line.chars()) {
            if char.is_ascii_digit() {
                current.value = (current.value * 10)
                    + char.to_string().parse::<u32>().expect("should be a number");

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
        vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ]
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
