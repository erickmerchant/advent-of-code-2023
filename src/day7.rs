use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    cards: (char, char, char, char, char),
    hand_strength: usize,
    score: usize,
}

pub fn part1(input: Vec<String>) -> usize {
    let mut hands = Vec::new();

    for line in input {
        let (cards, score) = line
            .split(' ')
            .collect_tuple()
            .expect("should be a tuple of two");
        let score = score.parse::<usize>().expect("should be a number");
        let cards = cards.chars().map(|c| match c {
            'T' => 'A',
            'J' => 'B',
            'Q' => 'C',
            'K' => 'D',
            'A' => 'E',
            _ => c,
        });
        let mut card_frequency_map = HashMap::new();

        for card in cards.clone() {
            if card == '0' {
                continue;
            }

            *card_frequency_map.entry(card).or_insert(1) *= 10;
        }

        let hand_strength = match card_frequency_map.values().sum::<usize>() / 10 {
            // 10000 => 10000, // Five of a kind
            // 1001 => 1001, // Four of a kind
            // 110 => 110, // Full house
            // 102 => 102, // Three of a kind
            // 21 => 21, // Two pair
            // 13 => 13, // One pair
            // 5 => 5, // High card
            1000 => 10000,
            101 => 1001,
            20 => 110,
            12 => 102,
            4 => 13,

            100 => 10000,
            11 => 1001,
            3 => 102,

            10 => 10000,
            2 => 1001,

            1 => 10000,
            0 => 10000,

            sum => sum,
        };

        let cards = cards
            .into_iter()
            .collect_tuple::<(char, char, char, char, char)>()
            .expect("should be a tuple of 5");

        hands.push(Hand {
            cards,
            hand_strength,
            score,
        });
    }

    hands.sort_by_key(|k| k.cards.4);
    hands.sort_by_key(|k| k.cards.3);
    hands.sort_by_key(|k| k.cards.2);
    hands.sort_by_key(|k| k.cards.1);
    hands.sort_by_key(|k| k.cards.0);
    hands.sort_by_key(|k| k.hand_strength);

    let mut total = 0;

    for (rank, hand) in (1..).zip(hands) {
        total += rank * hand.score;
    }

    total
}

pub fn part2(input: Vec<String>) -> usize {
    let input = input
        .iter()
        .map(|line| line.replace('J', "0"))
        .collect::<Vec<_>>();

    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fixture() -> Vec<String> {
        "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"
            .split('\n')
            .map(|s| s.trim().to_string())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_fixture()), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_fixture()), 5905);
    }
}
