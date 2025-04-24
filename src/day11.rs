type GalaxyList = Vec<Galaxy>;

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    row: usize,
    col: usize,
}

pub fn part1(input: Vec<String>) -> usize {
    get_total(input, 2)
}

pub fn part2(input: Vec<String>) -> usize {
    get_total(input, 1_000_000)
}

fn get_total(input: Vec<String>, rate: usize) -> usize {
    let mut cols = vec![rate; input[0].len()];
    let mut rows = vec![rate; input.len()];

    for (row, cells) in (0..).zip(input.clone()) {
        for (col, cell) in (0..).zip(cells.chars()) {
            if cell == '#' {
                cols[col] = 1;
                rows[row] = 1;
            }
        }
    }

    let mut galaxies = GalaxyList::new();
    let mut total = 0;

    for (origin_row, cells) in (0..).zip(input.clone()) {
        let row = rows[0..origin_row].iter().sum::<usize>();

        for (origin_col, cell) in (0..).zip(cells.chars()) {
            let col = cols[0..origin_col].iter().sum::<usize>();

            if cell == '#' {
                for galaxy in galaxies.clone() {
                    let mut sub_total = galaxy.col.abs_diff(col);

                    sub_total += row - galaxy.row;

                    total += sub_total;
                }

                galaxies.push(Galaxy { row, col });
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_get_total() {
        assert_eq!(get_total(get_fixture(), 2), 374);

        assert_eq!(get_total(get_fixture(), 10), 1030);

        assert_eq!(get_total(get_fixture(), 100), 8410);
    }
}
