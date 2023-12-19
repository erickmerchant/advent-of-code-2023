use rayon::prelude::*;

pub fn part1(input: Vec<String>) -> usize {
    get_result(input, 0)
}

pub fn part2(input: Vec<String>) -> usize {
    get_result(input, 1)
}

fn get_result(input: Vec<String>, target: usize) -> usize {
    let input = input.join("\n");
    let input = input.split("\n\n").collect::<Vec<_>>();

    input
        .par_iter()
        .map(|block| {
            let lines = block.split('\n').collect::<Vec<_>>();
            let mut columns = Vec::new();
            let mut rows = Vec::new();

            for (y, line) in (0..).zip(lines.clone()) {
                rows.push(0);

                for (x, c) in (0..).zip(line.chars()) {
                    if columns.len() <= x {
                        columns.push(0);
                    }

                    if c == '#' {
                        rows[y] |= 1 << x;

                        columns[x] |= 1 << y;
                    }
                }
            }

            'o: for i in 0..rows.len() - 1 {
                let diff: usize = rows[i] ^ rows[i + 1];
                let diff = diff.count_ones() as usize;

                if diff == 0 || diff == target {
                    let mut current = 0;

                    for (a, b) in (0..=i).rev().zip(i + 1..) {
                        if let (Some(a), Some(b)) = (rows.get(a), rows.get(b)) {
                            let diff = (a ^ b as &usize).count_ones() as usize;

                            current += diff;
                        }
                    }

                    if current != target {
                        continue 'o;
                    }

                    return (i + 1) * 100;
                }
            }

            'o: for i in 0..columns.len() - 1 {
                let diff: usize = columns[i] ^ columns[i + 1];
                let diff = diff.count_ones() as usize;

                if diff == 0 || diff == target {
                    let mut current = 0;

                    for (a, b) in (0..=i).rev().zip(i + 1..) {
                        if let (Some(a), Some(b)) = (columns.get(a), columns.get(b)) {
                            let diff = (a ^ b as &usize).count_ones() as usize;

                            current += diff;
                        }
                    }

                    if current != target {
                        continue 'o;
                    }

                    return i + 1;
                }
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 400);
    }
}
