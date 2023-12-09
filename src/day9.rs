use rayon::prelude::*;

pub fn part1(input: Vec<String>) -> usize {
    let sum: isize = input
        .par_iter()
        .map(|line| {
            let numbers: Vec<isize> = line
                .split(' ')
                .map(|s| s.parse::<isize>().expect("should be a valid number"))
                .collect();

            let mut results = vec![numbers];

            loop {
                let prev_numbers = results.last().expect("should have a last result");
                let mut numbers: Vec<isize> = vec![];

                for (i, n) in (0..).zip(prev_numbers) {
                    if i < prev_numbers.len() - 1 {
                        numbers.push(prev_numbers[i + 1] - n)
                    }
                }

                results.push(numbers.to_owned());

                if numbers.iter().all(|n| 0 == *n) {
                    break;
                }
            }

            results.reverse();

            let mut result = 0;

            for r in results {
                let last = r.last().expect("should have a last result");

                result += last.to_owned();
            }

            result
        })
        .sum();

    sum as usize
}

pub fn part2(input: Vec<String>) -> usize {
    let input: Vec<String> = input
        .iter()
        .map(|line| {
            let mut split: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();

            split.reverse();

            split.join(" ")
        })
        .collect();

    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 2);
    }
}
