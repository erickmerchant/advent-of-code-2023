use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

pub fn part1(input: Vec<String>) -> usize {
    get_result(input)
}

pub fn part2(_input: Vec<String>) -> usize {
    0
}

fn get_result(input: Vec<String>) -> usize {
    let re = Regex::new(r"\.+").expect("should be a valid regex");

    let result = input
        .par_iter()
        .map(|line| {
            let mut count = 0;
            let parts = line.split(' ').map(|s| s.to_string()).collect::<Vec<_>>();
            let (a_map, b_map) = parts
                .iter()
                .collect_tuple::<(&String, &String)>()
                .expect("a tuple of two");
            let b_map = b_map
                .split(',')
                .map(|s| s.parse::<usize>().expect("should be a number"))
                .collect_vec();
            let slots = a_map.match_indices("?").map(|(i, _)| i);
            let items = a_map.match_indices("#").collect::<Vec<_>>();
            let missing_count = b_map.iter().sum::<usize>() - items.len();
            let target_string = b_map.iter().map(|n| "#".repeat(*n)).join(" ");

            for combo in slots.combinations(missing_count) {
                let mut c_map = a_map.clone();

                for i in combo {
                    c_map.replace_range(i..=i, "#");
                }

                let c_map = c_map.replace("?", ".");
                let c_map = re.replace_all(c_map.as_str(), " ").to_string();
                let c_map = c_map.trim_matches('.').to_string();
                let c_map = c_map.trim_matches(' ').to_string();

                if c_map == target_string {
                    count += 1;
                }
            }

            count
        })
        .sum::<usize>();

    result
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
    fn test_get_result() {
        assert_eq!(get_result(get_fixture()), 21);
    }
}
