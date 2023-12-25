use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

type Key = (usize, usize, bool);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    weight: usize,
    edges: Vec<Vec<Key>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Limits {
    x: usize,
    y: usize,
    min: usize,
    max: usize,
}

fn get_edges(row: usize, column: usize, lateral: bool, limits: Limits) -> Vec<Vec<Key>> {
    let edges: Vec<Vec<Key>> = match lateral {
        false => {
            let mut columns = vec![];

            for size in limits.min..=limits.max {
                if column as isize > size as isize - 1 {
                    columns.push((1..=size).map(|i| column - i).collect_vec());
                }

                if (column as isize) < limits.x as isize - (size as isize - 1) {
                    columns.push((1..=size).map(|i| column + i).collect_vec());
                }
            }

            columns
                .iter()
                .map(|keys| {
                    keys.iter()
                        .map(|column| (row.to_owned(), column.to_owned(), lateral))
                        .collect_vec()
                })
                .collect()
        }
        true => {
            let mut rows = vec![];

            for size in limits.min..=limits.max {
                if row as isize > size as isize - 1 {
                    rows.push((1..=size).map(|i| row - i).collect_vec());
                }

                if (row as isize) < limits.y as isize - (size as isize - 1) {
                    rows.push((1..=size).map(|i| row + i).collect_vec());
                }
            }

            rows.iter()
                .map(|keys| {
                    keys.iter()
                        .map(|row| (row.to_owned(), column.to_owned(), lateral))
                        .collect_vec()
                })
                .collect()
        }
    };

    edges
}

pub fn part1(input: Vec<String>) -> usize {
    dijkstra(input, 1, 3)
}

pub fn part2(input: Vec<String>) -> usize {
    dijkstra(input, 4, 10)
}

pub fn dijkstra(input: Vec<String>, min: usize, max: usize) -> usize {
    let limits = Limits {
        x: input[0].len() - 1,
        y: input.len() - 1,
        min,
        max,
    };
    let mut nodes = HashMap::<Key, Node>::new();
    let mut unvisited = HashSet::<Key>::new();
    let mut distances = HashMap::<Key, usize>::new();

    for (row_number, row) in input.iter().enumerate() {
        for (column_number, column) in row.chars().enumerate() {
            let weight = column.to_digit(10).unwrap() as usize;

            for lateral in [true, false] {
                nodes.insert(
                    (row_number, column_number, lateral),
                    Node {
                        weight,
                        edges: get_edges(row_number, column_number, !lateral, limits),
                    },
                );

                unvisited.insert((row_number, column_number, lateral));

                if row_number == 0 && column_number == 0 {
                    distances.insert((row_number, column_number, lateral), 0);
                } else {
                    distances.insert((row_number, column_number, lateral), usize::MAX);
                }
            }
        }
    }

    while !unvisited.is_empty() {
        let temp_unvisited = unvisited.clone();
        let current = temp_unvisited
            .iter()
            .min_by_key(|&key| distances.get(key).unwrap_or(&usize::MAX))
            .expect("should have a min");
        let distance = distances[current];

        if current == &(limits.x, limits.y, true) || current == &(limits.x, limits.y, false) {
            break;
        }

        if distance == usize::MAX {
            break;
        }

        let new_distances = nodes[current]
            .clone()
            .edges
            .par_iter()
            .map(|edges| {
                let mut tentative_distance = distance;
                let mut new_distances = HashMap::<Key, usize>::new();

                for (i, edge) in edges.iter().enumerate() {
                    tentative_distance += nodes[&edge].weight;

                    if i + 1 >= limits.min && tentative_distance < distances[&edge] {
                        new_distances.insert(*edge, tentative_distance);
                    }
                }

                new_distances
            })
            .flatten()
            .collect::<HashMap<_, _>>();

        for (key, distance) in new_distances {
            distances.insert(key, distance);
        }

        unvisited.remove(current);
    }

    let result = [
        distances[&(limits.x, limits.y, true)],
        distances[&(limits.x, limits.y, false)],
    ]
    .iter()
    .map(|&distance| distance.to_owned())
    .collect::<Vec<usize>>();
    let result = result.iter().min().expect("should have a min");

    result.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        r"2413432311323
          3215453535623
          3255245654254
          3446585845452
          4546657867536
          1438598798454
          4457876987766
          3637877979653
          4654967986887
          4564679986453
          1224686865563
          2546548887735
          4322674655533"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 94);
    }
}
