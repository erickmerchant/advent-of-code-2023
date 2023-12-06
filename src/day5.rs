use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;

pub fn part1(input: Vec<String>) -> u32 {
    get_lowest(parse_input1(input))
}

pub fn part2(input: Vec<String>) -> u32 {
    get_lowest(parse_input2(input))
}

fn get_lowest((seeds, map_line_list_list): (Vec<u32>, Vec<MapLineList>)) -> u32 {
    let lowest = seeds
        .par_iter()
        .map(|seed| {
            let mut needle = *seed as isize;

            for map_line_list in map_line_list_list.clone() {
                let matching_map_line = map_line_list
                    .clone()
                    .into_iter()
                    .find(|map_line| needle >= map_line.min && needle < map_line.max);
                let modifier = match matching_map_line.clone() {
                    Some(map_line) => map_line.modifier,
                    None => 0_isize,
                };
                let new_needle = needle + modifier;

                if new_needle >= 0 {
                    needle = new_needle;
                } else {
                    panic!("should not be negative")
                }
            }

            needle
        })
        .min()
        .expect("should be a valid number");

    lowest as u32
}

#[derive(Debug, Clone)]
struct MapLine {
    min: isize,
    max: isize,
    modifier: isize,
}

type MapLineList = Vec<MapLine>;

fn parse_input1(input: Vec<String>) -> (Vec<u32>, Vec<MapLineList>) {
    let seeds = parse_seeds1(input[0].to_owned());
    let maps = parse_maps(input[1..].to_owned());

    (seeds, maps)
}

fn parse_input2(input: Vec<String>) -> (Vec<u32>, Vec<MapLineList>) {
    let seeds = parse_seeds2(input[0].to_owned());
    let maps = parse_maps(input[1..].to_owned());

    (seeds, maps)
}

fn parse_maps(input: Vec<String>) -> Vec<MapLineList> {
    let steps = input
        .join("\n")
        .split("\n\n")
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    let mut maps: Vec<MapLineList> = Vec::new();

    for step in steps {
        let mut current_map = MapLineList::new();

        for (i, line) in (0_u32..).zip(
            step.split('\n')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
        ) {
            if i == 0 {
                current_map = MapLineList::new();

                continue;
            }

            let (to, from, length) = line
                .split(' ')
                .map(|s| s.parse::<u32>().expect("should be a valid u32"))
                .collect_tuple()
                .expect("should be a tuple of three numbers");

            current_map.push(MapLine {
                min: from as isize,
                max: ((from as isize) + (length as isize)),
                modifier: (to as isize) - (from as isize),
            });
        }

        maps.push(current_map);
    }

    maps
}

fn parse_seeds1(line: String) -> Vec<u32> {
    static SEED_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"seeds: (?P<seeds>.*)").expect("should be a valid regex"));

    let captures = SEED_REGEX
        .captures(line.as_str())
        .expect("should be able to capture");

    captures["seeds"]
        .split(' ')
        .map(|s| s.parse::<u32>().expect("should be a valid u32"))
        .collect()
}

fn parse_seeds2(line: String) -> Vec<u32> {
    let seeds = parse_seeds1(line);
    let mut new_seeds: Vec<u32> = Vec::new();

    for chunk in seeds.chunks(2) {
        if chunk.len() != 2 {
            panic!("should be a chunk of two");
        }

        let mut range: Vec<u32> = (chunk[0]..(chunk[0] + chunk[1])).collect();

        new_seeds.append(&mut range);
    }

    new_seeds
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
            "
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "seeds: 79

                seed-to-soil map:
                50 98 2
                52 50 48
                "
                .split('\n')
                .map(|s| s.trim().to_string())
                .collect()
            ),
            81
        );

        assert_eq!(part1(get_fixture()), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 46);
    }
}
