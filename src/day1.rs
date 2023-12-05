pub fn part1(input: Vec<String>) -> u32 {
    let mut collection = Vec::<u32>::new();

    for line in input {
        collection.push(parse_numbers(line));
    }

    let result = &collection.iter().sum::<u32>();

    *result
}

pub fn part2(input: Vec<String>) -> u32 {
    let mut collection = Vec::<u32>::new();

    for line in input {
        collection.push(parse_numbers_plus(line));
    }

    let result = &collection.iter().sum::<u32>();

    *result
}

fn parse_numbers(line: String) -> u32 {
    let filtered = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<char>>();
    let first = filtered
        .first()
        .expect("should have a first")
        .to_string()
        .parse::<u32>()
        .expect("should parse a number here");
    let last = filtered
        .last()
        .expect("should have a last")
        .to_string()
        .parse::<u32>()
        .expect("should parse a number here");

    (first * 10) + last
}

fn parse_numbers_plus(line: String) -> u32 {
    let line = line
        .replace("zero", "zero0zero")
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    parse_numbers(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture1() -> Vec<(String, u32)> {
        vec![
            ("1abc2".to_string(), 12),
            ("pqr3stu8vwx".to_string(), 38),
            ("a1b2c3d4e5f".to_string(), 15),
            ("treb7uchet".to_string(), 77),
        ]
    }

    #[test]
    fn test_parse_numbers() {
        let fixture1 = get_fixture1();

        for (line, expected) in fixture1 {
            assert_eq!(parse_numbers(line), expected);
        }
    }

    fn get_fixture2() -> Vec<(String, u32)> {
        vec![
            ("two1nine".to_string(), 29),
            ("eightwothree".to_string(), 83),
            ("abcone2threexyz".to_string(), 13),
            ("xtwone3four".to_string(), 24),
            ("4nineeightseven2".to_string(), 42),
            ("zoneight234".to_string(), 14),
            ("7pqrstsixteen".to_string(), 76),
        ]
    }

    #[test]
    fn test_parse_numbers_plus() {
        let fixture2 = get_fixture2();

        for (line, expected) in fixture2 {
            assert_eq!(parse_numbers_plus(line), expected);
        }
    }

    #[test]
    fn test_part1() {
        let fixture1 = get_fixture1()
            .into_iter()
            .map(|(line, _)| line)
            .collect::<Vec<String>>();

        assert_eq!(part1(fixture1), 142);
    }

    #[test]
    fn test_part2() {
        let fixture2 = get_fixture2()
            .into_iter()
            .map(|(line, _)| line)
            .collect::<Vec<String>>();

        assert_eq!(part2(fixture2), 281);
    }
}
