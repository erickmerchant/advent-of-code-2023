use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Colors {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: usize,
    highs: Colors,
}

pub fn part1(input: Vec<String>) -> usize {
    let maxes = parse_colors("12 red cubes, 13 green cubes, and 14 blue cubes".to_string());
    let mut collection = Vec::new();

    for line in input {
        let game = parse_game(line);

        if game.highs.red <= maxes.red
            && game.highs.green <= maxes.green
            && game.highs.blue <= maxes.blue
        {
            collection.push(game.id);
        }
    }

    let result = &collection.iter().sum();

    *result
}

pub fn part2(input: Vec<String>) -> usize {
    let mut collection = Vec::new();

    for line in input {
        let game = parse_game(line);

        collection.push(game.highs.red * game.highs.green * game.highs.blue);
    }

    let result = &collection.iter().sum();

    *result
}

fn parse_colors(line: String) -> Colors {
    static COLOR_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(\d+) (red|blue|green)").expect("should be a valid regex"));

    let mut colors = Colors {
        red: 0,
        green: 0,
        blue: 0,
    };

    for (_, [count, color]) in COLOR_REGEX.captures_iter(&line).map(|c| c.extract()) {
        let count = count.parse::<usize>().expect("should be a valid usize");

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

fn parse_game(line: String) -> Game {
    static GAME_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"Game (?P<id>\d+): (?P<colors>.*)").expect("should be a valid regex")
    });

    let captures = GAME_REGEX
        .captures(line.as_str())
        .expect("should be able to capture");
    let id = &captures["id"]
        .parse::<usize>()
        .expect("should be a valid usize");

    Game {
        id: *id,
        highs: parse_colors(captures["colors"].to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let input = get_fixture();

        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = get_fixture();

        assert_eq!(part2(input), 2286);
    }
}
