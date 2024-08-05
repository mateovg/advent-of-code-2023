use std::i32;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result = parse_input(input)
        .iter()
        .filter(|g| g.valid())
        .map(|g| g.0)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_input(input);
    let result = games
        .iter()
        .map(|g| g.min_cubes())
        .map(|min_set| Game::power_set(min_set))
        .sum();
    Some(result)
}

#[derive(Debug, PartialEq)]
struct Game(u32, Vec<(i32, i32, i32)>);
impl Game {
    fn new(input: &str) -> Game {
        let (id_split, round_split) = input.split_once(':').unwrap();

        let id = id_split[5..].parse::<u32>().unwrap();
        let rounds = round_split
            .split(';')
            .map(|round| {
                let mut round_data = (0, 0, 0);
                round.split(',').for_each(|item| {
                    let parts: Vec<&str> = item.trim().split_whitespace().collect();
                    let color = parts[1].chars().next().unwrap();
                    let count = parts[0].parse::<i32>().unwrap();
                    match color {
                        'r' => round_data.0 = count,
                        'g' => round_data.1 = count,
                        'b' => round_data.2 = count,
                        _ => panic!("Not possible"),
                    }
                });
                round_data
            })
            .collect();

        Game(id, rounds)
    }

    fn valid(&self) -> bool {
        for round in &self.1 {
            if round.0 > 12 || round.1 > 13 || round.2 > 14 {
                return false;
            }
        }
        true
    }

    fn min_cubes(&self) -> (i32, i32, i32) {
        self.1.iter().fold((0, 0, 0), |acc, s| {
            (acc.0.max(s.0), acc.1.max(s.1), acc.2.max(s.2))
        })
    }

    fn power_set(set: (i32, i32, i32)) -> u32 {
        (set.0 * set.1 * set.2) as u32
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(|line| Game::new(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
