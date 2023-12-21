use rayon::prelude::*;

pub fn part1(input: Vec<String>) -> usize {
    let values = input[0]
        .split(',')
        .map(|s| s.as_bytes().iter().map(|b| *b as u128).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let result = values
        .par_iter()
        .map(|v| v.iter().fold(0, hash))
        .sum::<u128>();

    result as usize
}

pub fn part2(_input: Vec<String>) -> usize {
    0
}

fn hash(acc: u128, curr: &u128) -> u128 {
    let mut res = acc + *curr;

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
