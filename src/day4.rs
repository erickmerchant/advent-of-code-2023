use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

pub type NumberSet = HashSet<u32>;

#[derive(Debug, Eq, PartialEq)]
pub struct Game {
    pub id: u32,
    pub winning: NumberSet,
    pub actual: NumberSet,
}

pub fn parse_game(line: String) -> Game {
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

    #[test]
    fn test_parse_game() {
        let game = parse_game("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string());
        let mut winning = NumberSet::new();
        let mut actual = NumberSet::new();

        winning.insert(41);
        winning.insert(48);
        winning.insert(83);
        winning.insert(86);
        winning.insert(17);
        actual.insert(83);
        actual.insert(86);
        actual.insert(6);
        actual.insert(31);
        actual.insert(17);
        actual.insert(9);
        actual.insert(48);
        actual.insert(53);

        assert_eq!(
            game,
            Game {
                id: 1,
                winning,
                actual
            }
        );
    }
}
