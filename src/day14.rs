use std::collections::{hash_map::Entry::Vacant, HashMap};

pub fn part1(input: Vec<String>) -> usize {
    let mut tilter = Tilter {
        state: input.clone(),
    };

    tilter.tilt_north();

    tilter.get_weight()
}

pub fn part2(input: Vec<String>) -> usize {
    let mut tilter = Tilter {
        state: input.clone(),
    };

    tilter.spin_cycle(1_000_000_000);

    tilter.get_weight()
}

#[derive(Default)]
struct Tilter {
    state: Vec<String>,
}

impl Tilter {
    fn get_weight(&mut self) -> usize {
        let mut result = 0;
        let state = self.state.clone();
        let input_length = self.state.len();

        for (weight, line) in (1..=input_length).rev().zip(state) {
            for c in line.chars() {
                if c == 'O' {
                    result += weight;
                }
            }
        }

        result
    }

    fn spin_cycle(&mut self, cycles: usize) {
        let mut map = HashMap::<Vec<String>, usize>::new();
        let mut loop_found = false;
        let mut count = 0;

        loop {
            if count >= cycles {
                break;
            }

            if !loop_found {
                if let Vacant(e) = map.entry(self.state.clone()) {
                    e.insert(count);
                } else {
                    let loop_length =
                        count - map.get(&self.state.clone()).expect("should have a number");

                    count += loop_length * ((cycles - count) / loop_length) - loop_length;

                    loop_found = true;
                }
            }

            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();

            count += 1;
        }
    }

    fn tilt_north(&mut self) {
        let mut columns = HashMap::new();
        let mut output = self.state.clone();
        let state = self.state.clone();

        for (row, line) in (0..).zip(state) {
            for (column, c) in (0..).zip(line.chars()) {
                if c == '#' {
                    columns.insert(column, row);
                } else if c == 'O' {
                    output[row].replace_range(column..column + 1, ".");

                    let row = columns.get(&column);

                    if let Some(row) = row {
                        output[row + 1].replace_range(column..column + 1, "O");

                        columns.insert(column, row + 1);
                    } else {
                        output[0].replace_range(column..column + 1, "O");

                        columns.insert(column, 0);
                    }
                }
            }
        }

        self.state = output;
    }

    fn tilt_south(&mut self) {
        let mut columns = HashMap::new();
        let mut output = self.state.clone();
        let input_length = self.state.len();
        let state = self.state.clone();

        for (row, line) in (0..input_length).zip(state).rev() {
            for (column, c) in (0..).zip(line.chars()) {
                if c == '#' {
                    columns.insert(column, row);
                } else if c == 'O' {
                    output[row].replace_range(column..column + 1, ".");

                    let row = columns.get(&column);

                    if let Some(row) = row {
                        output[row - 1].replace_range(column..column + 1, "O");

                        columns.insert(column, row - 1);
                    } else {
                        output[input_length - 1].replace_range(column..column + 1, "O");

                        columns.insert(column, input_length - 1);
                    }
                }
            }
        }

        self.state = output;
    }

    fn tilt_east(&mut self) {
        let mut output = self.state.clone();
        let state = self.state.clone();

        for (row, line) in (0..).zip(state) {
            let mut next = None;

            for (column, c) in (0..line.len())
                .zip(line.chars())
                .collect::<Vec<_>>()
                .iter()
                .rev()
            {
                let column = *column;
                let c = *c;

                if c == '#' {
                    next = Some(column);
                } else if c == 'O' {
                    output[row].replace_range(column..column + 1, ".");

                    if let Some(column) = next {
                        output[row].replace_range(column - 1..column, "O");

                        next = Some(column - 1);
                    } else {
                        output[row].replace_range(line.len() - 1..line.len(), "O");

                        next = Some(line.len() - 1);
                    }
                }
            }
        }

        self.state = output;
    }

    fn tilt_west(&mut self) {
        let mut output = self.state.clone();
        let state = self.state.clone();

        for (row, line) in (0..).zip(state) {
            let mut next = None;

            for (column, c) in (0..).zip(line.chars()) {
                let column = column.to_owned();
                let c = c.to_owned();

                if c == '#' {
                    next = Some(column);
                } else if c == 'O' {
                    output[row].replace_range(column..column + 1, ".");

                    if let Some(column) = next {
                        output[row].replace_range(column + 1..column + 2, "O");

                        next = Some(column + 1);
                    } else {
                        output[row].replace_range(0..1, "O");

                        next = Some(0);
                    }
                }
            }
        }

        self.state = output;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture1() -> Vec<String> {
        "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }
    fn get_fixture2() -> Vec<String> {
        ".....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #...O###.O
        #.OOO#...O"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture1()), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture1()), 64);
    }

    #[test]
    fn test_spin_cycle() {
        let mut tilter = Tilter {
            state: get_fixture1(),
        };

        tilter.spin_cycle(3);

        assert_eq!(tilter.state, get_fixture2());
    }
}
