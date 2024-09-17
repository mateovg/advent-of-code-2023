use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let result = parse_input(input).iter().map(|g| g.get_score()).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let games: Vec<ScratchCard> = parse_input(input)
        .iter()
        .map(|g| ScratchCard {
            id: g.id,
            wins: g.get_wins(),
            copies: 1,
        })
        .collect();

    let result = (0..games.len())
        .fold(vec![1; games.len()], |mut copies, i| {
            let card = &games[i];
            let card_copies = copies[i];
            for j in i + 1..std::cmp::min(games.len(), i + card.wins as usize + 1) {
                copies[j] += card_copies;
            }
            copies
        })
        .iter()
        .sum();
    Some(result)
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    id: usize,
    winning_numbers: HashSet<usize>,
    player_numbers: HashSet<usize>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct ScratchCard {
    id: usize,
    wins: usize,
    copies: usize,
}

impl Game {
    fn get_wins(&self) -> usize {
        self.player_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(&n))
            .collect::<Vec<&usize>>()
            .len() as usize
    }

    fn get_score(&self) -> usize {
        let wins = self.get_wins();
        match wins {
            0 => 0,
            1 => 1,
            n => usize::pow(2, n - 1),
        }
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(|l| parse_line(l)).collect()
}

fn parse_line(line: &str) -> Game {
    let number = line
        .split(':')
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap();
    dbg!(&number);
    let id = number.parse().unwrap();
    let winning_numbers = line.split(&[':', '|']).nth(1).unwrap();
    let player_numbers = line.rsplit('|').nth(0).unwrap();

    let winning_numbers = winning_numbers
        .split_whitespace()
        .filter_map(|n| Some(n.parse::<usize>()))
        .map(|n| n.unwrap())
        .collect();

    let player_numbers = player_numbers
        .split_whitespace()
        .filter_map(|n| Some(n.parse::<usize>()))
        .map(|n| n.unwrap())
        .collect();

    Game {
        id,
        winning_numbers,
        player_numbers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_parse_line() {
        let result = parse_line("Card 1: 41 48 83 86 177 | 83 86 6 31 17 9 48 53");
        let expected = Game {
            id: 1,
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
            player_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };
        assert_eq!(result.id, expected.id);
        assert_eq!(result.winning_numbers.len(), expected.winning_numbers.len());
        assert_eq!(result.player_numbers.len(), expected.player_numbers.len());
    }

    #[test]
    fn test_get_wins() {
        let game = Game {
            id: 1,
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
            player_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };
        let result = game.get_wins();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_score() {
        let game = Game {
            id: 1,
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
            player_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };
        let result = game.get_score();
        assert_eq!(result, 8);
    }
}
