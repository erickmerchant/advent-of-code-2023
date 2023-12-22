use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Move = (usize, usize, Direction);
type MoveList = Vec<Move>;

pub fn part1(input: Vec<String>) -> usize {
    let grid = Grid {
        cells: input
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    };

    grid.get_energized(0, 0, Direction::Right)
}

pub fn part2(input: Vec<String>) -> usize {
    let mut starts = MoveList::new();

    let grid = Grid {
        cells: input
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    };

    for x in 0..grid.get_x_max() {
        starts.push((x, 0, Direction::Down));
        starts.push((x, grid.get_y_max(), Direction::Up));
    }

    for y in 0..grid.get_y_max() {
        starts.push((0, y, Direction::Right));
        starts.push((grid.get_x_max(), y, Direction::Left));
    }

    let result = starts
        .par_iter()
        .map(|(x, y, dir)| grid.get_energized(*x, *y, *dir))
        .max()
        .expect("should have a max");

    result
}

struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn get_y_max(&self) -> usize {
        self.cells.len() - 1
    }

    fn get_x_max(&self) -> usize {
        self.cells[0].len() - 1
    }

    fn get_energized(&self, x: usize, y: usize, dir: Direction) -> usize {
        let y_max = self.get_y_max();
        let x_max = self.get_x_max();
        let mut prev_moves = HashSet::<Move>::new();
        let mut energized = HashSet::<(usize, usize)>::new();
        let mut moves: MoveList = self.get_new_moves(x, y, dir);

        while !moves.is_empty() {
            let mut new_moves = MoveList::new();

            for (x, y, dir) in moves {
                if prev_moves.contains(&(x, y, dir)) {
                    continue;
                }

                prev_moves.insert((x, y, dir));
                energized.insert((x, y));

                let new_coords = match dir {
                    Direction::Up if y > 0 => Some((x, y - 1)),
                    Direction::Down if y < y_max => Some((x, y + 1)),
                    Direction::Left if x > 0 => Some((x - 1, y)),
                    Direction::Right if x < x_max => Some((x + 1, y)),
                    _ => None,
                };

                if let Some((x, y)) = new_coords {
                    new_moves.append(self.get_new_moves(x, y, dir).as_mut())
                }
            }

            moves = new_moves;
        }

        energized.len()
    }

    fn get_new_moves(&self, x: usize, y: usize, dir: Direction) -> MoveList {
        let mut new_moves = MoveList::new();
        match (self.cells[y][x], dir) {
            ('|', Direction::Left) | ('|', Direction::Right) => {
                new_moves.push((x, y, Direction::Up));
                new_moves.push((x, y, Direction::Down));
            }
            ('-', Direction::Up) | ('-', Direction::Down) => {
                new_moves.push((x, y, Direction::Left));
                new_moves.push((x, y, Direction::Right));
            }
            ('/', Direction::Up) => {
                new_moves.push((x, y, Direction::Right));
            }
            ('/', Direction::Down) => {
                new_moves.push((x, y, Direction::Left));
            }
            ('/', Direction::Left) => {
                new_moves.push((x, y, Direction::Down));
            }
            ('/', Direction::Right) => {
                new_moves.push((x, y, Direction::Up));
            }
            ('\\', Direction::Up) => {
                new_moves.push((x, y, Direction::Left));
            }
            ('\\', Direction::Down) => {
                new_moves.push((x, y, Direction::Right));
            }
            ('\\', Direction::Left) => {
                new_moves.push((x, y, Direction::Up));
            }
            ('\\', Direction::Right) => {
                new_moves.push((x, y, Direction::Down));
            }
            _ => {
                new_moves.push((x, y, dir));
            }
        }

        new_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 51);
    }
}
