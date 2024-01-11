pub fn part1(input: Vec<String>) -> usize {
    get_steps(input, 64)
}

pub fn part2(_input: Vec<String>) -> usize {
    0
}

pub fn get_steps(_input: Vec<String>, _steps: usize) -> usize {
    0
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
