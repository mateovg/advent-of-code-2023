advent_of_code::solution!(7);
use std::{cmp::Ordering, collections::HashMap};
use itertools::{Itertools};

#[derive(Debug, PartialEq, Hash, Clone, Eq, Copy)]
struct Card {
    value: char,
}
impl Card {
    fn to_rank(&self) -> usize {
        match self.value {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            '*' => 0,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
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
            HandRanks::FiveOfAKind => 6,
            HandRanks::FourOfAKind => 5,
            HandRanks::FullHouse => 4,
            HandRanks::ThreeOfAKind => 3,
            HandRanks::TwoPair => 2,
            HandRanks::OnePair => 1,
            HandRanks::HighCard => 0,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
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
    fn from_string_jokers(s: &str) -> Hand {
        let mut parts = s.split_whitespace();
        let cards: Vec<Card> = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                if c == 'J'{
                    return Card {value: '*'};
                }
                Card { value: c }
            })
            .collect();
        let bid: u32 = parts.next().unwrap().parse().unwrap();
        Hand { cards, bid }
    }
    fn get_counts(&self) -> HashMap<Card, usize> {
        self.cards.clone().into_iter().counts()
    }

    fn get_rank(&self) -> HandRanks {
        let mut counts = self.get_counts();
        let jokers = counts.remove(&Card{value: '*'}).unwrap_or(0);
        if jokers == 5 { return HandRanks::FiveOfAKind};

        let mut counts = counts.into_values().collect::<Vec<usize>>();
        counts.sort_unstable_by(|a,b| b.cmp(a));
        counts[0] += jokers;

        match counts[0]{
            5..=10 => HandRanks::FiveOfAKind,
            4 => HandRanks::FourOfAKind,
            3 => {
                if counts[1] == 2{
                    HandRanks::FullHouse
                } else{
                    HandRanks::ThreeOfAKind
                }
            }
            2 => {
                if counts[1] == 2{
                    HandRanks::TwoPair
                } else {
                    HandRanks::OnePair
                }
            }
            1 => HandRanks::HighCard,
            _ =>  unreachable!()
        }
    }

    fn to_rank(&self) -> usize {
        self.get_rank().to_rank()
    }

    fn card_rank(&self, idx: usize) -> usize{
        self.cards[idx].to_rank()
    }
}


impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cards == other.cards {
            return Ordering::Equal;
        }
        let mut candidate = self.to_rank().cmp(&other.to_rank());
        let mut i = 0;
        while candidate == Ordering::Equal{
            candidate = self.card_rank(i).cmp(&other.card_rank(i));
            i += 1;
        }
        candidate
    }
}



pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = parse_input(input);
    hands.sort();
    hands.iter().enumerate().fold(0, |acc, (rank, hand)| {
        acc + hand.bid * (rank as u32 + 1)
    }
    ).into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = parse_input_two(input);
    // for hand in &hands{
    //     assert_eq!(&0, hand.get_counts().get(&Card{value: 'J'}).unwrap_or(&0));
    // }
    hands.sort();
    hands.iter().enumerate().fold(0, |acc, (rank, hand)| {
        acc + hand.bid * (rank as u32 + 1)
    }
    ).into()
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(|l| Hand::from_string(l)).collect()
}

fn parse_input_two(input: &str) -> Vec<Hand> {
    input.lines().map(|l| Hand::from_string_jokers(l)).collect()
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
        assert_eq!(result, Some(5905));
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

    #[test]
    fn test_hand_cmp() {
        let hand_one = Hand::from_string("32T3K 765");
        let hand_two = Hand::from_string("T55J5 684");
        assert_eq!(hand_one.to_rank(), 1);
        assert_eq!(hand_two.to_rank(), 3);
        assert!(hand_two > hand_one);
    }
}
