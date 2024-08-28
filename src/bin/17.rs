use std::{isize, u32};

advent_of_code::solution!(17);

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Point {
    x: i32,
    y: i32,
}

struct Crucicle {
    position: Option<Point>,
    direction: Direction,
    momentum: u8,
    heat_loss: u32,
    grid: Vec<Vec<u32>>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let crucible = parse_input(input);
    crucible.heat_loss()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Crucicle {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    Crucicle {
        position: Some(Point { x: 0, y: 0 }),
        direction: Direction::Right,
        momentum: 0,
        heat_loss: 0,
        grid,
    }
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
    fn turn_right(&self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

// impl Point {
//     fn step(&self, direction: Direction) -> Option<Point> {
//         let (mut x, mut y) = (self.x, self.y);
//         match direction {
//             Direction::Up => y -= 1,
//             Direction::Down => y += 1,
//             Direction::Left => x -= 1,
//             Direction::Right => x += 1,
//         }
//         Point { x, y }
//     }
// }

impl Crucicle {
    fn step(&mut self) -> Option<Point> {
        let direction = self.direction.clone();
        if self.momentum == 3 {
            return None;
        }
        let (mut x, mut y) = if let Some(p) = &self.position {
            match direction {
                Direction::Up => (p.x, p.y - 1),
                Direction::Down => (p.x, p.y + 1),
                Direction::Left => (p.x - 1, p.y),
                Direction::Right => (p.x + 1, p.y),
            }
        } else {
            return None;
        };
        if x < 0 || x > self.grid[0].len() as i32 || y < 0 || y > self.grid.len() as i32 {
            return None;
        }
        Some(Point { x, y })
    }

    /// For each state check, we can turn l, turn right, or continue
    /// State = loc, dir, moves_in_dir, heat_loss
    /// return min heatloss, return at bottom right
    fn heat_loss(&self) -> Option<u32> {
        let (r, c) = (self.grid.len(), self.grid[0].len());

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.grid.len(), 13);
        assert_eq!(result.grid[0].len(), 13);
    }
}
