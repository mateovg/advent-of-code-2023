use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}
impl Race {
    fn does_win(&self, held_down: u64) -> bool {
        let speed = held_down;
        let time_remaining = self.time - held_down;
        self.distance < time_remaining * speed
    }

    fn ways_to_win(&self) -> u64 {
        (0..self.time)
            .filter(|t| self.does_win(*t))
            .map(|_| 1)
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (time, distance) = parse_input(input);
    let races = create_races(time, distance);
    Some(races.iter().map(|r| r.ways_to_win()).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    // len of a digit is log10(d)+1
    let (time, distance) = parse_input(input);
    let time = concat_digits(time);
    let distance = concat_digits(distance);

    let race = Race { time, distance };
    Some(race.ways_to_win())
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut times: Vec<u64> = Vec::new();
    let mut distance: Vec<u64> = Vec::new();

    for line in input.lines() {
        match line {
            l if l.starts_with("Time:") => {
                times = l
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
            }
            l if l.starts_with("Distance:") => {
                distance = l
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
            }
            _ => {}
        }
    }

    (times, distance)
}

fn create_races(time: Vec<u64>, distance: Vec<u64>) -> Vec<Race> {
    time.iter()
        .zip(distance)
        .map(|(t, d)| Race {
            time: *t,
            distance: d,
        })
        .collect()
}

fn concat_digits(numbers: Vec<u64>) -> u64 {
    numbers
        .iter()
        .fold(0, |total, t| total * 10u64.pow(t.ilog10() + 1) + *t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }

    #[test]
    fn test_does_win() {
        let race = Race {
            time: 30,
            distance: 200,
        };
        assert_eq!(race.does_win(10), false);
        assert_eq!(race.does_win(11), true);
        assert_eq!(race.does_win(19), true);
        assert_eq!(race.does_win(20), false);
    }

    #[test]
    fn test_concat_digits() {
        let result = concat_digits(vec![1, 2, 3, 4, 10]);
        assert_eq!(result, 123410);
    }
}
