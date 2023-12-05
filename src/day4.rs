use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

pub fn part1(input: Vec<String>) -> u32 {
    let mut collection = Vec::<u32>::new();

    for line in input {
        let game = parse_game(line);
        let win_count = game.winning.intersection(&game.actual).count() as u32;
        let points = if win_count > 0 {
            2_u32.pow(win_count - 1)
        } else {
            0_u32
        };

        collection.push(points);
    }

    let result = &collection.iter().sum::<u32>();

    *result
}

pub fn part2(input: Vec<String>) -> u32 {
    let mut collection: Vec<u32> = vec![1; input.len()];

    for (row, line) in (0_u32..).zip(input) {
        let game = parse_game(line);
        let win_count = game.winning.intersection(&game.actual).count() as u32;

        for i in (row + 1)..=(win_count + row) {
            if i < collection.len() as u32 {
                collection[i as usize] += collection[row as usize];
            }
        }
    }

    let result = &collection.iter().sum::<u32>();

    *result
}

type NumberSet = HashSet<u32>;

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    winning: NumberSet,
    actual: NumberSet,
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
        .parse::<u32>()
        .expect("should be a valid u32");

    Game {
        id: *id,
        winning: captures["winning"]
            .split(' ')
            .map(|c| c.parse::<u32>().expect("should be a valid u32"))
            .collect::<NumberSet>(),
        actual: captures["actual"]
            .split(' ')
            .map(|c| c.parse::<u32>().expect("should be a valid u32"))
            .collect::<NumberSet>(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ]
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
