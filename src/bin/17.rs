use std::{cmp::Ordering, collections::BinaryHeap};

advent_of_code::solution!(17);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Crucible {
    position: Option<Point>,
    direction: Direction,
    momentum: u8,
    heat_loss: usize,
}

struct CrucibleSolver {
    grid: Vec<Vec<usize>>,
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let crucible = parse_input(input);
    Some(crucible.heat_loss(0, 3))
}

pub fn part_two(input: &str) -> Option<usize> {
    let crucible = parse_input(input);
    Some(crucible.heat_loss(4, 10))
}

fn parse_input(input: &str) -> CrucibleSolver {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    CrucibleSolver { grid }
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

impl Crucible {
    fn step(&mut self) -> () {
        let direction = self.direction.clone();

        let (x, y) = if let Some(p) = &self.position {
            match direction {
                Direction::Up => (p.x, p.y - 1),
                Direction::Down => (p.x, p.y + 1),
                Direction::Left => (p.x - 1, p.y),
                Direction::Right => (p.x + 1, p.y),
            }
        } else {
            self.position = None;
            return;
        };

        self.position = Some(Point { x, y })
    }
}

impl CrucibleSolver {
    fn heat_loss(&self, min_momentum: u8, max_momentun: u8) -> usize {
        // BFS Search for minimum heat loss
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let target = Point {
            x: cols as isize - 1,
            y: rows as isize - 1,
        };

        let mut heap = BinaryHeap::new();
        let mut cost_map = std::collections::HashMap::new();

        // Start top left, going down and right
        heap.push(Crucible {
            position: Some(Point { x: 0, y: 0 }),
            direction: Direction::Down,
            momentum: 0,
            heat_loss: 0,
        });
        heap.push(Crucible {
            position: Some(Point { x: 0, y: 0 }),
            direction: Direction::Right,
            momentum: 0,
            heat_loss: 0,
        });

        // Essentially Dijkstra's
        while let Some(crucible) = heap.pop() {
            if let Some(p) = crucible.position {
                // If at final and able to stop, needed for part two
                if p.x == target.x && p.y == target.y && crucible.momentum >= min_momentum {
                    return crucible.heat_loss;
                }

                let state_key = (p.x, p.y, crucible.direction.clone(), crucible.momentum);
                if let Some(&new_heat_loss) = cost_map.get(&state_key) {
                    if crucible.heat_loss > new_heat_loss {
                        continue;
                    }
                }

                let possible_moves = [
                    (crucible.direction.clone(), crucible.momentum + 1),
                    (crucible.direction.turn_left(), 1),
                    (crucible.direction.turn_right(), 1),
                ];

                for (new_direction, new_momentum) in possible_moves.iter() {
                    if *new_momentum > max_momentun {
                        continue;
                    }

                    if crucible.momentum < min_momentum && *new_direction != crucible.direction {
                        continue;
                    }

                    let mut new_crucible = Crucible {
                        position: crucible.position,
                        direction: new_direction.clone(),
                        momentum: *new_momentum,
                        heat_loss: crucible.heat_loss,
                    };
                    new_crucible.step();

                    if let Some(new_position) = new_crucible.position {
                        if new_position.x < 0
                            || new_position.x >= cols as isize
                            || new_position.y < 0
                            || new_position.y >= rows as isize
                        {
                            continue;
                        }

                        new_crucible.heat_loss +=
                            self.grid[new_position.y as usize][new_position.x as usize];
                        let new_state = (
                            new_position.x,
                            new_position.y,
                            new_direction.clone(),
                            *new_momentum,
                        );
                        if new_crucible.heat_loss < *cost_map.get(&new_state).unwrap_or(&usize::MAX) {
                            cost_map.insert(new_state, new_crucible.heat_loss);
                            heap.push(new_crucible);
                        }
                    }
                }
            }
        }

        usize::max_value()
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
        assert_eq!(result, Some(94));
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(part_two(input), Some(71));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.grid.len(), 13);
        assert_eq!(result.grid[0].len(), 13);
    }
}
