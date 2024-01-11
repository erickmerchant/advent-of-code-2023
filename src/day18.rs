use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone)]
struct Instruction {
    direction: char,
    length: isize,
}

#[derive(Debug)]
struct Node {
    start: (isize, isize),
    end: (isize, isize),
    toggle: bool,
}

pub fn part1(input: Vec<String>) -> usize {
    static DIG_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?P<direction>U|D|R|L) (?P<length>\d+) \(#(?P<color>[a-f0-9]{6})\)")
            .expect("should be a valid regex")
    });

    let mut instructions = Vec::new();

    for line in input {
        let captures = DIG_REGEX
            .captures(line.as_str())
            .expect("should be able to capture");

        let length = &captures["length"]
            .parse::<isize>()
            .expect("should be a valid number");

        let direction = &captures["direction"]
            .chars()
            .next()
            .expect("should be a valid char");

        instructions.push(Instruction {
            direction: *direction,
            length: *length,
        });
    }

    get_size(instructions)
}

pub fn part2(input: Vec<String>) -> usize {
    static DIG_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(U|D|R|L) (\d+) \(#(?P<length>[a-f0-9]{5})(?P<direction>\d{1})\)")
            .expect("should be a valid regex")
    });

    let mut instructions = Vec::new();

    for line in input {
        let captures = DIG_REGEX
            .captures(line.as_str())
            .expect("should be able to capture");

        let length =
            isize::from_str_radix(&captures["length"], 16).expect("should be a valid number");

        let direction = &captures["direction"]
            .chars()
            .next()
            .expect("should be a valid char");

        let direction = match direction {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!("should be a valid direction"),
        };

        instructions.push(Instruction { direction, length });
    }

    get_size(instructions)
}

fn get_size(instructions: Vec<Instruction>) -> usize {
    let mut nodes: Vec<Node> = vec![];
    let mut row = 0;
    let mut col = 0;
    let mut previous = (row, col);
    let mut direction_sequence = vec![];
    let mut previous_direction = instructions
        .last()
        .expect("should have a last instruction")
        .direction;
    let mut min_row = 0;
    let mut max_row = 0;
    let mut min_col = 0;
    let mut max_col = 0;

    for instruction in instructions {
        match instruction.direction {
            'U' => {
                row -= instruction.length;

                if direction_sequence
                    .last()
                    .expect("should have a last direction")
                    == &'D'
                    && ['L', 'R'].contains(&previous_direction)
                {
                    let last = nodes.last_mut().expect("should have a last node");

                    last.toggle = false;
                }
            }
            'D' => {
                row += instruction.length;

                if direction_sequence
                    .last()
                    .expect("should have a last direction")
                    == &'U'
                    && ['L', 'R'].contains(&previous_direction)
                {
                    let last = nodes.last_mut().expect("should have a last node");

                    last.toggle = false;
                }
            }
            'L' => {
                col -= instruction.length;
            }
            'R' => {
                col += instruction.length;
            }
            _ => panic!("should be a valid direction"),
        }

        if previous.1 > col {
            nodes.push(Node {
                start: (row, col),
                end: previous,
                toggle: true,
            });
        } else {
            nodes.push(Node {
                start: previous,
                end: (row, col),
                toggle: true,
            });
        }

        previous = (row, col);

        direction_sequence.push(previous_direction);
        previous_direction = instruction.direction;

        if row < min_row {
            min_row = row;
        }

        if row > max_row {
            max_row = row;
        }

        if col < min_col {
            min_col = col;
        }

        if col > max_col {
            max_col = col;
        }
    }

    let mut result = 0;

    for row in min_row..=max_row {
        let mut bounds = nodes
            .iter()
            .filter(|node| {
                (node.start.0 == row && node.end.0 == row)
                    || (node.start.1 == node.end.1
                        && (node.start.0 > row && node.end.0 < row
                            || node.start.0 < row && node.end.0 > row))
            })
            .map(|node| (node.toggle, node.start.1, node.end.1))
            .sorted_by_key(|node| node.1)
            .collect_vec();

        if bounds.is_empty() {
            bounds.push((false, min_col, max_col));
        }

        let first_bound = bounds.first().expect("should have a first bound");
        let mut inside = first_bound.0;

        result += first_bound.2 - first_bound.1 + 1;

        let mut prev_end = first_bound.2 + 1;

        for bound in bounds.iter().skip(1) {
            if inside {
                result += bound.1 - prev_end;
            }

            result += bound.2 - bound.1 + 1;

            prev_end = bound.2 + 1;

            if bound.0 {
                inside = !inside;
            }
        }
    }

    result.unsigned_abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        r"R 6 (#70c710)
          D 5 (#0dc571)
          L 2 (#5713f0)
          D 2 (#d2c081)
          R 2 (#59c680)
          D 2 (#411b91)
          L 5 (#8ceee2)
          U 2 (#caa173)
          L 1 (#1b58a2)
          U 2 (#caa171)
          R 2 (#7807d2)
          U 3 (#a77fa3)
          L 2 (#015232)
          U 2 (#7a21e3)"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 952_408_144_115);
    }
}
