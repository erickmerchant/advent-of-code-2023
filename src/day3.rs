use std::collections::{HashMap, HashSet};

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Number {
    pub value: u32,
    pub field: CoordsSet,
}

pub type Coords = (u32, u32);
pub type CoordsSet = HashSet<Coords>;
type SymbolMap = HashMap<Coords, char>;
type NumberList = Vec<Number>;
type Output = (SymbolMap, NumberList);

pub fn parse_input(input: Vec<String>) -> Output {
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

    #[test]
    fn test_parse_input() {
        let output = parse_input(vec![
            "...".to_string(),
            ".12".to_string(),
            ".*.".to_string(),
        ]);
        let mut expected_symbols = SymbolMap::new();
        let mut expected_numbers = NumberList::new();

        expected_symbols.insert((2, 1), '*');

        let mut field = CoordsSet::new();

        field.insert((0, 0));
        field.insert((0, 1));
        field.insert((0, 2));
        field.insert((0, 3));
        field.insert((1, 0));
        field.insert((1, 1));
        field.insert((1, 2));
        field.insert((1, 3));
        field.insert((2, 0));
        field.insert((2, 1));
        field.insert((2, 2));
        field.insert((2, 3));

        expected_numbers.push(Number {
            value: 12_u32,
            field,
        });

        assert_eq!(output, (expected_symbols, expected_numbers));
    }
}
