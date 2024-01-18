use itertools::Itertools;
use rayon::prelude::*;

pub fn part1(input: Vec<String>) -> usize {
    let values = input[0]
        .split(',')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let result = values
        .par_iter()
        .map(|v| v.iter().fold(0_u128, hash))
        .sum::<u128>();

    result as usize
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum OperationType {
    Remove,
    Insert(u128),
}

#[derive(Debug, Eq, PartialEq)]
struct Operation {
    label: Vec<u128>,
    hash: u128,
    operation_type: OperationType,
}

type Item = (Vec<u128>, u128);

pub fn part2(input: Vec<String>) -> usize {
    let values = input[0]
        .split(',')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let operations = values
        .iter()
        .map(|values| {
            let mut operation = Operation {
                label: vec![],
                hash: 0,
                operation_type: OperationType::Remove,
            };

            for value in values {
                if *value == '-' {
                    break;
                } else if *value == '=' {
                    operation.operation_type = OperationType::Insert(0_u128);
                } else if let OperationType::Insert(len) = operation.operation_type {
                    operation.operation_type = OperationType::Insert(
                        (len * 10)
                            + value
                                .to_string()
                                .parse::<u128>()
                                .expect("should be parsable as number"),
                    )
                } else {
                    operation.label.push(*value as u128);

                    operation.hash = hash(operation.hash, value);
                }
            }

            operation
        })
        .collect::<Vec<_>>();
    let mut boxes = vec![Vec::<Item>::new(); 256];

    for operation in operations {
        match operation.operation_type {
            OperationType::Insert(len) => {
                if let Some((i, _)) = boxes[operation.hash as usize]
                    .iter()
                    .find_position(|(label, _len)| *label == operation.label)
                {
                    boxes[operation.hash as usize][i] = (operation.label, len);
                } else {
                    boxes[operation.hash as usize].push((operation.label, len));
                }
            }
            OperationType::Remove => {
                if let Some((i, _)) = boxes[operation.hash as usize]
                    .iter()
                    .find_position(|(label, _len)| *label == operation.label)
                {
                    boxes[operation.hash as usize].remove(i);
                }
            }
        }
    }

    let total = boxes
        .iter()
        .enumerate()
        .map(|(i, items)| {
            let mut subtotal = 0;

            for (j, (_, len)) in items.iter().enumerate() {
                subtotal += (i + 1) * (j + 1) * (*len as usize);
            }

            subtotal
        })
        .sum::<usize>();

    total
}

fn hash(acc: u128, curr: &char) -> u128 {
    let mut res = acc + (*curr as u128);

    res *= 17;

    res %= 256;

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 145);
    }
}
