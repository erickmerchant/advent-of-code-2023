pub fn parse_numbers(line: String) -> u32 {
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

pub fn parse_numbers_plus(line: String) -> u32 {
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

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers("1abc2".to_string()), 12);

        assert_eq!(parse_numbers("pqr3stu8vwx".to_string()), 38);

        assert_eq!(parse_numbers("a1b2c3d4e5f".to_string()), 15);

        assert_eq!(parse_numbers("treb7uchet".to_string()), 77);
    }

    #[test]
    fn test_parse_numbers_plus() {
        assert_eq!(parse_numbers_plus("two1nine".to_string()), 29);

        assert_eq!(parse_numbers_plus("eightwothree".to_string()), 83);

        assert_eq!(parse_numbers_plus("abcone2threexyz".to_string()), 13);

        assert_eq!(parse_numbers_plus("xtwone3four".to_string()), 24);

        assert_eq!(parse_numbers_plus("4nineeightseven2".to_string()), 42);

        assert_eq!(parse_numbers_plus("zoneight234".to_string()), 14);

        assert_eq!(parse_numbers_plus("7pqrstsixteen".to_string()), 76);
    }
}
