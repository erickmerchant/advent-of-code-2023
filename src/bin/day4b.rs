use advent::{day4::*, read_stdin};

fn main() {
    let lines: Vec<String> = read_stdin().flatten().collect();

    let result = aggregate_scratch_its(lines);

    println!("{result:?}");
}

fn aggregate_scratch_its(lines: Vec<String>) -> u32 {
    let mut collection: Vec<u32> = vec![1; lines.len()];

    for (row, line) in (0_u32..).zip(lines) {
        let game = parse_game(line);
        let win_count = game.winning.intersection(&game.actual).count() as u32;

        for i in (row + 1)..=(win_count + row) {
            if i < collection.len() as u32 {
                collection[i as usize] += collection[row as usize];
            }
        }
    }

    let result = &collection.iter().sum::<u32>();

    result.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_scratch_its() {
        assert_eq!(
            aggregate_scratch_its(vec![
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
                "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
                "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
                "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
                "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
                "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string()
            ]),
            30
        );
    }
}
