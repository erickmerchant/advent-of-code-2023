pub fn part1(input: Vec<String>) -> usize {
    let collection = input.iter().map(|line| parse_numbers(line.as_str()));

    let result = &collection.sum::<usize>();

    *result
}

pub fn part2(input: Vec<String>) -> usize {
    let collection = input.iter().map(|line| parse_numbers_plus(line.as_str()));

    let result = &collection.sum::<usize>();

    *result
}

fn parse_numbers(line: &str) -> usize {
    let filtered = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<char>>();
    let first = filtered
        .first()
        .expect("should have a first")
        .to_string()
        .parse::<usize>()
        .expect("should parse a number here");
    let last = filtered
        .last()
        .expect("should have a last")
        .to_string()
        .parse::<usize>()
        .expect("should parse a number here");

    (first * 10) + last
}

fn parse_numbers_plus(line: &str) -> usize {
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

    parse_numbers(&line)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture1() -> Vec<String> {
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    fn get_fixture2() -> Vec<String> {
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        let fixture1 = get_fixture1();

        assert_eq!(part1(fixture1), 142);
    }

    #[test]
    fn test_part2() {
        let fixture2 = get_fixture2();

        assert_eq!(part2(fixture2), 281);
    }
}
