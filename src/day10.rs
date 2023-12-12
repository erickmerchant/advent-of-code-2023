use itertools::Itertools;

pub fn part1(input: Vec<String>) -> usize {
    let (_, path) = get_path(input);

    path.len() / 2
}

pub fn part2(input: Vec<String>) -> usize {
    let line_length = input.first().expect("should have a first line").len();
    let (first_letter, path) = get_path(input.clone());
    let grid = input
        .join("")
        .replace('S', first_letter.to_string().as_str())
        .chars()
        .collect::<Vec<_>>();
    let grid = grid.chunks(line_length).collect::<Vec<_>>();
    let boundaries = ['F', '7', 'L', 'J', '|'];
    let mut count = 0;

    for (row_number, row) in (0..).zip(grid) {
        let mut inside = false;
        let mut prev_path_cell = '|';

        for (cell_number, cell) in (0..).zip(row) {
            let grid_number = row_number * line_length + cell_number;

            if path.contains(&grid_number) {
                if boundaries.contains(cell) {
                    inside = match (prev_path_cell, cell) {
                        ('F', 'J') => !inside,
                        ('L', '7') => !inside,
                        (_, '|') => !inside,
                        _ => inside,
                    };
                    prev_path_cell = *cell;
                }
            } else if inside {
                count += 1;
            }
        }
    }

    count
}

fn get_path(input: Vec<String>) -> (char, Vec<usize>) {
    let line_length = input.first().expect("should have a first line").len() as isize;
    let neg_line_length = 0 - line_length;
    let grid = input.join("").chars().collect::<Vec<_>>();
    let start = grid
        .iter()
        .position(|c| *c == 'S')
        .expect("should have a start position") as isize;
    let mut path = Vec::new();
    let start_connections = [
        (-1, ['-', 'L', 'F']),
        (1, ['-', 'J', '7']),
        (neg_line_length, ['|', '7', 'F']),
        (line_length, ['|', 'L', 'J']),
    ];
    let start_connections = start_connections
        .iter()
        .filter_map(|(direction, valids)| {
            if valids.contains(grid.get((start + direction) as usize).unwrap_or(&'#')) {
                Some(direction)
            } else {
                None
            }
        })
        .sorted()
        .collect_tuple::<(&isize, &isize)>()
        .expect("should have two possible next positions");
    let first_letter = match (start_connections.0, start_connections.1) {
        (a, -1) if *a == neg_line_length => 'J',
        (a, 1) if *a == neg_line_length => 'L',
        (a, b) if *a == neg_line_length && *b == line_length => '|',
        (-1, 1) => '-',
        (-1, b) if *b == line_length => '7',
        (1, b) if *b == line_length => 'F',
        _ => panic!("should have a first letter"),
    };
    let mut current_and_direction = (start, *start_connections.0);

    loop {
        let current = current_and_direction.0;
        let direction = current_and_direction.1;

        path.push((current + direction) as usize);

        let next_direction = match grid.get((current + direction) as usize) {
            Some('-') => direction,
            Some('|') => direction,
            Some('L') => {
                if direction == -1 {
                    neg_line_length
                } else {
                    1
                }
            }
            Some('J') => {
                if direction == 1 {
                    neg_line_length
                } else {
                    -1
                }
            }
            Some('7') => {
                if direction == 1 {
                    line_length
                } else {
                    -1
                }
            }
            Some('F') => {
                if direction == -1 {
                    line_length
                } else {
                    1
                }
            }
            Some('S') => break,
            _ => panic!("off the grid"),
        };

        current_and_direction = (current + direction, next_direction);
    }

    (first_letter, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture1() -> Vec<String> {
        "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    fn get_fixture2() -> Vec<String> {
        "..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        .........."
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    fn get_fixture3() -> Vec<String> {
        "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ.F7FJ-
        L---JF-JLJ....FJLJJ7
        |F|F-JF---7...L7L|7|
        |FFJF7L7F-JF7..L---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture1()), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture2()), 4);
        assert_eq!(part2(get_fixture3()), 10);
    }
}
