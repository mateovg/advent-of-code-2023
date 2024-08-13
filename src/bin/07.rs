advent_of_code::solution!(7);
use std::{cmp::Ordering, collections::HashMap, sync::atomic::Ordering};

use itertools::{zip, Itertools};

#[derive(Debug, PartialEq, Hash, Clone, Eq, Copy)]
struct Card {
    value: char,
}
impl Card {
    fn to_rank(&self) -> usize {
        match self.value {
            'A' => 0,
            'K' => 1,
            'Q' => 2,
            'J' => 3,
            'T' => 4,
            '9' => 5,
            '8' => 6,
            '7' => 7,
            '6' => 8,
            '5' => 9,
            '4' => 10,
            '3' => 11,
            '2' => 12,
            _ => 100,
        }
    }
}

#[derive(Debug, PartialEq, Hash, Clone, Eq, Copy)]
enum HandRanks {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
impl HandRanks {
    fn to_rank(&self) -> usize {
        match self {
            HandRanks::FiveOfAKind => 0,
            HandRanks::FourOfAKind => 1,
            HandRanks::FullHouse => 2,
            HandRanks::ThreeOfAKind => 3,
            HandRanks::TwoPair => 4,
            HandRanks::OnePair => 5,
            HandRanks::HighCard => 6,
        }
    }
}

#[derive(Debug, Hash, Clone, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}
impl Hand {
    fn from_string(s: &str) -> Hand {
        let mut parts = s.split_whitespace();
        let cards: Vec<Card> = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| Card { value: c })
            .collect();
        let bid: u32 = parts.next().unwrap().parse().unwrap();
        Hand { cards, bid }
    }

    fn get_counts(&self) -> HashMap<Card, usize> {
        self.cards.clone().into_iter().counts()
    }

    fn get_rank(&self) -> HandRanks {
        let counts = self.get_counts();
        let card_counts = counts.values().counts().clone();
        if card_counts.contains_key(&5) {
            return HandRanks::FiveOfAKind;
        } else if card_counts.contains_key(&4) {
            return HandRanks::FourOfAKind;
        } else if card_counts.contains_key(&3) && card_counts.contains_key(&2) {
            return HandRanks::FullHouse;
        } else if card_counts.contains_key(&3) {
            return HandRanks::ThreeOfAKind;
        } else if card_counts.contains_key(&2) {
            if card_counts[&2] == 2 {
                return HandRanks::TwoPair;
            }
            return HandRanks::OnePair;
        }
        return HandRanks::HighCard;
    }

    fn to_rank(&self) -> usize {
        self.get_rank().to_rank()
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.to_rank() == other.to_rank() {
            for (a, b) in zip(self.cards, other.cards) {
                if a != b {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = parse_input(input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(|l| Hand::from_string(l)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].bid, 765);
        assert_eq!(result[2].bid, 28);
    }

    #[test]
    fn test_hand_from_strong() {
        let test_hand = "32T3K 765";
        let result = Hand::from_string(test_hand);
        assert_eq!(
            result,
            Hand {
                cards: vec![
                    Card { value: '3' },
                    Card { value: '2' },
                    Card { value: 'T' },
                    Card { value: '3' },
                    Card { value: 'K' },
                ],
                bid: 765,
            }
        )
    }

    #[test]
    fn test_hand_get_counts() {
        let result = Hand::from_string("32T3K 765").get_counts();
        let expected = HashMap::from([
            (Card { value: 'K' }, 1),
            (Card { value: 'T' }, 1),
            (Card { value: '3' }, 2),
            (Card { value: '2' }, 1),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_hand_get_rank() {
        let hand = Hand::from_string("32T3K 765");
        assert_eq!(hand.get_rank(), HandRanks::OnePair);
        let hand = Hand::from_string("T55J5 684");
        assert_eq!(hand.get_rank(), HandRanks::ThreeOfAKind);
        let hand = Hand::from_string("KK677 430");
        assert_eq!(hand.get_rank(), HandRanks::TwoPair);
        let hand = Hand::from_string("KTJ32 304");
        assert_eq!(hand.get_rank(), HandRanks::HighCard);
        let hand = Hand::from_string("QQQKK 400");
        assert_eq!(hand.get_rank(), HandRanks::FullHouse);
    }
}
