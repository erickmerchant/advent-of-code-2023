use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub struct Colors {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Game {
    pub id: u32,
    pub highs: Colors,
}

pub fn parse_colors(line: String) -> Colors {
    static COLOR_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(\d+) (red|blue|green)").expect("should be a valid regex"));

    let mut colors = Colors {
        red: 0,
        green: 0,
        blue: 0,
    };

    for (_, [count, color]) in COLOR_REGEX.captures_iter(&line).map(|c| c.extract()) {
        let count = count.parse::<u32>().expect("should be a valid u32");

        match color {
            "red" => {
                colors.red = if count > colors.red {
                    count
                } else {
                    colors.red
                }
            }
            "green" => {
                colors.green = if count > colors.green {
                    count
                } else {
                    colors.green
                }
            }
            "blue" => {
                colors.blue = if count > colors.blue {
                    count
                } else {
                    colors.blue
                }
            }
            _ => panic!("should be a valid color"),
        }
    }

    colors
}

pub fn parse_game(line: String) -> Game {
    static GAME_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"Game (?P<id>\d+): (?P<colors>.*)").expect("should be a valid regex")
    });

    let captures = GAME_REGEX
        .captures(line.as_str())
        .expect("should be able to capture");

    let id = &captures["id"]
        .parse::<u32>()
        .expect("should be a valid u32");

    Game {
        id: *id,
        highs: parse_colors(captures["colors"].to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let game = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());

        assert_eq!(
            game,
            Game {
                id: 1,
                highs: Colors {
                    red: 4,
                    green: 2,
                    blue: 6,
                }
            }
        );

        let game = parse_game(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        );

        assert_eq!(
            game,
            Game {
                id: 2,
                highs: Colors {
                    red: 1,
                    green: 3,
                    blue: 4,
                }
            }
        );

        let game = parse_game(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
        );

        assert_eq!(
            game,
            Game {
                id: 3,
                highs: Colors {
                    red: 20,
                    green: 13,
                    blue: 6,
                }
            }
        );

        let game = parse_game(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
        );

        assert_eq!(
            game,
            Game {
                id: 4,
                highs: Colors {
                    red: 14,
                    green: 3,
                    blue: 15,
                }
            }
        );

        let game = parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string());

        assert_eq!(
            game,
            Game {
                id: 5,
                highs: Colors {
                    red: 6,
                    green: 3,
                    blue: 2
                }
            }
        );
    }
}
