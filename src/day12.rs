use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> usize {
    get_total(input, 1) as usize
}

pub fn part2(input: Vec<String>) -> usize {
    get_total(input, 5) as usize
}

fn get_total(input: Vec<String>, fold: u128) -> u128 {
    let total = input
        .par_iter()
        .map(|line| {
            let parts = line.split(' ').map(|s| s.to_string()).collect::<Vec<_>>();
            let (left, right) = parts
                .iter()
                .collect_tuple::<(&String, &String)>()
                .expect("a tuple of two");
            let left = vec![left; fold as usize].iter().join("?");
            let length = left.len() as u128;
            let right = vec![right; fold as usize].iter().join(",");
            let blocks = right
                .split(',')
                .map(|s| s.parse::<u128>().expect("should be a number"))
                .collect_vec();
            let mut pattern = 0;
            let mut anti_pattern = 0;

            for (i, c) in (0..).zip(left.chars()) {
                if c == '#' {
                    pattern |= 1 << i;
                } else if c == '.' {
                    anti_pattern |= 1 << i;
                }
            }

            let mut combinations = Combinations {
                pattern,
                anti_pattern,
                length,
                cache: HashMap::new(),
            };
            let total =
                combinations.calculate(0, 0, &blocks, blocks.iter().sum(), blocks.len() as u128);

            total
        })
        .sum::<u128>();

    total
}

struct Combinations {
    pattern: u128,
    anti_pattern: u128,
    length: u128,
    cache: HashMap<(u128, u128), u128>,
}

impl Combinations {
    pub fn calculate(
        &mut self,
        value: u128,
        min_pos: u128,
        blocks: &[u128],
        blocks_sum: u128,
        blocks_length: u128,
    ) -> u128 {
        if self.cache.contains_key(&(min_pos, blocks_length)) {
            return *self
                .cache
                .get(&(min_pos, blocks_length))
                .expect("cache should have a value");
        }

        let mut total = 0;
        let max_pos = self.length - (blocks_sum + blocks_length - 1);
        let block = (0..blocks[0]).map(|i| 1 << i).sum::<u128>();

        for pos in min_pos..=max_pos {
            let block = block << pos;
            let pattern_mask = if blocks_length == 1 {
                (0..self.length).map(|i| 1 << i).sum::<u128>()
            } else {
                (0..pos).map(|i| 1 << i).sum::<u128>() | block
            };
            let value = value | block;

            if (pattern_mask & self.anti_pattern) & value != 0 {
                continue;
            }

            if (pattern_mask & self.pattern) & value != pattern_mask & self.pattern {
                continue;
            }

            total += if blocks_length == 1 {
                1
            } else {
                self.calculate(
                    value,
                    pos + blocks[0] + 1,
                    &blocks[1..],
                    blocks_sum - blocks[0],
                    blocks_length - 1,
                )
            };
        }

        self.cache.insert((min_pos, blocks_length), total);

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_get_total() {
        assert_eq!(get_total(get_fixture(), 1), 21);
        assert_eq!(get_total(get_fixture(), 5), 525152);
    }
}
