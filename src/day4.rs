use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

type NumberSet = HashSet<usize>;

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: usize,
    winning: NumberSet,
    actual: NumberSet,
}

pub fn part1(input: Vec<String>) -> usize {
    let mut collection: Vec<usize> = Default::default();

    for line in input {
        let game = parse_game(line);
        let win_count = game.winning.intersection(&game.actual).count();
        let points = if win_count > 0 {
            1 << (win_count - 1)
        } else {
            0
        };

        collection.push(points);
    }

    let result = &collection.iter().sum();

    *result
}

pub fn part2(input: Vec<String>) -> usize {
    let mut collection: Vec<usize> = vec![1; input.len()];

    for (row, line) in (0..).zip(input) {
        let game = parse_game(line);
        let win_count = game.winning.intersection(&game.actual).count();

        for i in (row + 1)..=(win_count + row) {
            if i < collection.len() {
                collection[i] += collection[row];
            }
        }
    }

    let result = &collection.iter().sum();

    *result
}

fn parse_game(line: String) -> Game {
    let line = line.replace("  ", " ");

    static GAME_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"Card\s+(?P<id>\d+): (?P<winning>.*?) \| (?P<actual>.*)")
            .expect("should be a valid regex")
    });

    let captures = GAME_REGEX
        .captures(line.as_str())
        .expect("should be able to capture");
    let id = &captures["id"]
        .parse::<usize>()
        .expect("should be a valid usize");

    Game {
        id: *id,
        winning: captures["winning"]
            .split(' ')
            .map(|c| c.parse::<usize>().expect("should be a valid usize"))
            .collect(),
        actual: captures["actual"]
            .split(' ')
            .map(|c| c.parse::<usize>().expect("should be a valid usize"))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 30);
    }
}
