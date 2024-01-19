use std::collections::{HashMap, HashSet};

pub fn part1(input: Vec<String>) -> usize {
    get_steps(input, 64)
}

pub fn part2(_input: Vec<String>) -> usize {
    0
}

pub fn get_steps(input: Vec<String>, steps: usize) -> usize {
    let mut grid = HashMap::<(isize, isize), bool>::new();
    let mut plots = HashSet::<(isize, isize)>::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let is_plot = match c {
                '.' => true,
                'S' => {
                    plots.insert((x as isize, y as isize));

                    true
                }
                '#' => false,
                _ => panic!("Invalid character"),
            };

            grid.insert((x as isize, y as isize), is_plot);
        }
    }

    for _i in 0..steps {
        let mut new_plots = HashSet::<(isize, isize)>::new();

        for (x, y) in plots.clone() {
            if *grid.get(&(x - 1, y)).unwrap_or(&false) {
                new_plots.insert((x - 1, y));
            }

            if *grid.get(&(x + 1, y)).unwrap_or(&false) {
                new_plots.insert((x + 1, y));
            }

            if *grid.get(&(x, y - 1)).unwrap_or(&false) {
                new_plots.insert((x, y - 1));
            }

            if *grid.get(&(x, y + 1)).unwrap_or(&false) {
                new_plots.insert((x, y + 1));
            }
        }

        plots = new_plots;
    }

    plots.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        r"...........
          .....###.#.
          .###.##..#.
          ..#.#...#..
          ....#.#....
          .##..S####.
          .##..#...#.
          .......##..
          .##.#.####.
          .##..##.##.
          ..........."
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(get_steps(get_fixture(), 6), 16);
    }
}
