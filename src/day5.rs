use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;

type MapList = Vec<Map>;

#[derive(Debug, Clone)]
struct Map {
    a: isize,
    b: isize,
    len: isize,
}

pub fn part1(input: Vec<String>) -> usize {
    let seeds = parse_seeds1(input[0].to_owned());
    let maps = parse_maps(input[1..].to_owned());

    get_lowest1(seeds, maps)
}

pub fn part2(input: Vec<String>) -> usize {
    let seeds = parse_seeds2(input[0].to_owned());
    let mut maps = parse_maps(input[1..].to_owned());

    maps.reverse();

    get_lowest2(seeds, maps)
}

fn get_lowest1(seeds: Vec<isize>, maps: Vec<MapList>) -> usize {
    let locations = seeds
        .par_iter()
        .map(|seed| {
            let mut needle = *seed;

            for map_list in maps.clone() {
                let matching_map = map_list
                    .clone()
                    .into_iter()
                    .find(|map| needle >= map.b && needle < map.b + map.len);
                let modifier = match matching_map.clone() {
                    Some(map) => map.a - map.b,
                    None => 0,
                };
                let new_needle = needle + modifier;

                if new_needle >= 0 {
                    needle = new_needle;
                } else {
                    panic!("should not be negative")
                }
            }

            needle as usize
        })
        .collect::<Vec<_>>();
    let lowest = locations.iter().min().expect("should have a minimum");

    *lowest
}

const CHUNK_SIZE: isize = 1_000;

fn get_lowest2(seeds: MapList, maps: Vec<MapList>) -> usize {
    let mut lowest = usize::MAX;

    for chunk_start in 0_isize.. {
        let chunk = (chunk_start * CHUNK_SIZE..(chunk_start * CHUNK_SIZE) + CHUNK_SIZE - 1)
            .collect::<Vec<_>>();
        let matches = chunk
            .par_iter()
            .map(|start| {
                let mut needle = *start;

                for map_list in maps.clone() {
                    let matching_map = map_list
                        .clone()
                        .into_iter()
                        .find(|map| needle >= map.a && needle < map.a + map.len);
                    let modifier = match matching_map.clone() {
                        Some(map) => map.b - map.a,
                        None => 0,
                    };
                    let new_needle = needle + modifier;

                    if new_needle >= 0 {
                        needle = new_needle;
                    } else {
                        panic!("should not be negative")
                    }
                }

                if seeds
                    .clone()
                    .into_iter()
                    .any(|seed| needle >= seed.a && needle < seed.a + seed.len)
                {
                    Some(*start as usize)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let matches = matches.into_iter().flatten().collect::<Vec<_>>();

        if let Some(min) = matches.iter().min() {
            lowest = *min;

            break;
        }
    }

    lowest
}

fn parse_maps(input: Vec<String>) -> Vec<MapList> {
    let steps = input
        .join("\n")
        .split("\n\n")
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();
    let mut maps = Vec::new();

    for step in steps {
        let mut current_map = MapList::new();

        for (i, line) in (0..).zip(
            step.split('\n')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>(),
        ) {
            if i == 0 {
                current_map = MapList::new();

                continue;
            }

            let (a, b, len) = line
                .split(' ')
                .map(|s| s.parse::<isize>().expect("should be a valid usize"))
                .collect_tuple()
                .expect("should be a tuple of three numbers");

            current_map.push(Map { a, b, len });
        }

        maps.push(current_map);
    }

    maps
}

fn parse_seeds1(line: String) -> Vec<isize> {
    static SEED_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"seeds: (?P<seeds>.*)").expect("should be a valid regex"));

    let captures = SEED_REGEX
        .captures(line.as_str())
        .expect("should be able to capture");

    captures["seeds"]
        .split(' ')
        .map(|s| s.parse::<isize>().expect("should be a valid isize"))
        .collect()
}

fn parse_seeds2(line: String) -> MapList {
    let seeds = parse_seeds1(line);
    let mut maps = MapList::new();

    for chunk in seeds.chunks(2) {
        if chunk.len() != 2 {
            panic!("should be a chunk of two");
        }

        let (a, len) = chunk
            .iter()
            .collect_tuple()
            .expect("should be a tuple of two numbers");

        maps.push(Map {
            a: *a,
            len: *len,
            b: 0,
        });
    }

    maps
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
        56 93 4"
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
                52 50 48"
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
