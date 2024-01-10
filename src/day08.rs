use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> usize {
    let (steps, nodes) = parse_input(input);
    let steps_len = steps.len();
    let mut count = 0;
    let mut current = "AAA".to_string();

    loop {
        let (left, right) = nodes.get(&current).expect("should have a node");
        let direction = steps.get(count % steps_len).expect("should have a step");

        current = match direction.as_str() {
            "L" => left.to_owned(),
            "R" => right.to_owned(),
            _ => panic!("invalid direction"),
        };

        count += 1;

        if current == *"ZZZ" {
            break;
        }
    }

    count
}

pub fn part2(input: Vec<String>) -> usize {
    let (steps, nodes) = parse_input(input);
    let steps_len = steps.len();
    let currents = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();
    let numbers = currents
        .par_iter()
        .map(|current| {
            let mut count = 0;
            let mut current = *current;

            loop {
                let (left, right) = nodes.get(current).expect("should have a node");
                let direction = steps.get(count % steps_len).expect("should have a step");

                current = match direction.as_str() {
                    "L" => left,
                    "R" => right,
                    _ => panic!("invalid direction"),
                };

                count += 1;

                if current.ends_with('Z') {
                    break;
                }
            }

            count
        })
        .collect::<Vec<_>>();
    let mut count = numbers[0];

    for &number in numbers.iter().skip(1) {
        count = count * number / num::integer::gcd(count, number);
    }

    count
}

fn parse_input(input: Vec<String>) -> (Vec<String>, HashMap<String, (String, String)>) {
    let steps = input[0]
        .split("")
        .filter(|s| s != &"")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let mut nodes = HashMap::new();
    let node_re = Regex::new(r"(?P<name>\w+) = \((?P<left>\w+), (?P<right>\w+)\)")
        .expect("should be a valid regex");

    for node in input[2..].iter() {
        let parts = node_re.captures(node).expect("should match regex");
        nodes.insert(
            parts["name"].to_string(),
            (parts["left"].to_string(), parts["right"].to_string()),
        );
    }

    (steps, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture1() -> Vec<String> {
        "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    fn get_fixture2() -> Vec<String> {
        "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    fn get_fixture3() -> Vec<String> {
        "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture1()), 2);
        assert_eq!(part1(get_fixture2()), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture3()), 6);
    }
}
