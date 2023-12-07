use once_cell::sync::Lazy;
use regex::Regex;

pub fn part1(input: Vec<String>) -> usize {
    let times = parse_numbers(input[0].clone());
    let distances = parse_numbers(input[1].clone());
    let races: Vec<(usize, usize)> = times.into_iter().zip(distances).collect();

    get_result(races)
}

pub fn part2(input: Vec<String>) -> usize {
    let re = Regex::new(r"(\d)\s+").expect("should be a valid regex");
    let input: Vec<String> = input
        .iter()
        .map(|line| re.replace_all(line, "$1").to_string())
        .collect();

    part1(input)
}

fn get_result(races: Vec<(usize, usize)>) -> usize {
    let mut result = 1;

    for (time, distance) in races {
        let mut count = 0;

        for t in 0..=time {
            let d = (time - t) * t;

            if d > distance {
                count += 1;
            }
        }

        result *= count;
    }

    result
}

fn parse_numbers(line: String) -> Vec<usize> {
    static NUMBERS_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r".*?: (?P<numbers>.*)").expect("should be a valid regex"));

    let captures = NUMBERS_REGEX
        .captures(line.as_str())
        .expect("should be able to capture");

    captures["numbers"]
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().expect("should be a valid usize"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "Time:     7  15   30
        Distance:  9  40  200"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 71503);
    }
}
