use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Edge {
    direction: Direction,
    length: isize,
    start: Point,
    end: Point,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Grid {
    width: usize,
    height: usize,
    points: HashSet<Point>,
    edges: Vec<Edge>,
}

#[derive(Debug)]
struct EdgeInstruction(Direction, isize);

pub fn part_one(input: &str) -> Option<usize> {
    let edge_instrucs = parse_input(input);
    let mut grid = Grid::new(edge_instrucs);
    grid.fill_grid();
    // grid.print_grid();
    Some(grid.points.len() as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let instructions = parse_input_two(input);
    dbg!(&instructions);
    let mut grid = Grid::new(instructions);
    grid.fill_grid();
    Some(grid.points.len() as usize)
}

fn parse_input(input: &str) -> Vec<EdgeInstruction> {
    input
        .lines()
        .map(|l| {
            let (dir, length, _) = l.split_whitespace().collect_tuple().unwrap();
            let direction = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => panic!("Error parsing"),
            };
            EdgeInstruction(direction, length.parse().unwrap())
        })
        .collect()
}

fn parse_input_two(input: &str) -> Vec<EdgeInstruction> {
    input
        .lines()
        .map(|l| {
            let (_, _, instruction) = l.split_whitespace().collect_tuple().unwrap();
            let dir: u8 = instruction[7..8].parse().unwrap();
            let direction = match dir {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                _ => panic!("Error parsing"),
            };
            let length = usize::from_str_radix(&instruction[2..7], 16).unwrap();
            EdgeInstruction(direction, length as isize)
        })
        .collect()
}

impl Grid {
    fn new(edge_instrucs: Vec<EdgeInstruction>) -> Self {
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        let mut start = Point { x: 0, y: 0 };
        let mut points: HashSet<Point> = HashSet::from([start.clone()]);
        let mut edges = vec![];

        for instruct in &edge_instrucs {
            let edge = Edge::new(instruct, start);
            let curr_points = edge.line();
            for point in curr_points {
                points.insert(point);
            }
            start = edge.end.clone();
            edges.push(edge);
            max_x = max_x.max(start.x);
            min_x = min_x.min(start.x);
            max_y = max_y.max(start.y);
            min_y = min_y.min(start.y);
        }

        let points: HashSet<Point> = points.iter().map(|p| p.normalize(min_x, min_y)).collect();

        let edges = edges
            .iter()
            .map(|e| Edge {
                direction: e.direction,
                length: e.length,
                start: e.start.normalize(min_x, min_y),
                end: e.end.normalize(min_x, min_y),
            })
            .collect();

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        Self {
            width,
            height,
            points,
            edges,
        }
    }

    fn fill_grid(&mut self) -> &mut Self {
        let points = (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| Point {
                x: x as isize,
                y: y as isize,
            })
            .filter(|p| self.is_inside(p))
            .count();

        dbg!(points);
        self
    }

    fn is_inside(&self, point: &Point) -> bool {
        if self.points.contains(point) {
            return true;
        }
        let mut winding_number = 0;

        for edge in &self.edges {
            let start = edge.start.clone();
            let end = edge.end.clone();

            if (start.y <= point.y && end.y > point.y) || (start.y > point.y && end.y <= point.y) {
                let x_intersect =
                    start.x + (point.y - start.y) * (end.x - start.x) / (end.y - start.y);
                if x_intersect > point.x {
                    winding_number += if start.y < end.y { 1 } else { -1 };
                }
            }
        }

        winding_number != 0
    }

    fn print_grid(&self) {
        for y in 0..self.height {
            println!("");
            for x in 0..self.width {
                if self.points.contains(&Point {
                    x: x as isize,
                    y: y as isize,
                }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }
}

impl Edge {
    fn new(EdgeInstruction(direction, length): &EdgeInstruction, start: Point) -> Edge {
        let end = Edge::endpoint(*direction, *length, &start);
        Edge {
            direction: *direction,
            length: *length,
            start,
            end,
        }
    }

    fn line(&self) -> Vec<Point> {
        match self.direction {
            Direction::Up => (self.end.y..self.start.y)
                .map(|y| Point { x: self.start.x, y })
                .collect(),
            Direction::Down => (self.start.y..self.end.y)
                .map(|y| Point {
                    x: self.start.x,
                    y: y + 1,
                })
                .collect(),
            Direction::Left => (self.end.x..self.start.x)
                .map(|x| Point { x, y: self.start.y })
                .collect(),
            Direction::Right => (self.start.x..self.end.x)
                .map(|x| Point {
                    x: x + 1,
                    y: self.start.y,
                })
                .collect(),
        }
    }

    fn endpoint(direction: Direction, length: isize, start: &Point) -> Point {
        match direction {
            Direction::Up => Point {
                x: start.x,
                y: start.y - length,
            },
            Direction::Down => Point {
                x: start.x,
                y: start.y + length,
            },
            Direction::Left => Point {
                x: start.x - length,
                y: start.y,
            },
            Direction::Right => Point {
                x: start.x + length,
                y: start.y,
            },
        }
    }
}

impl Point {
    fn normalize(&self, min_x: isize, min_y: isize) -> Self {
        Self {
            x: self.x - min_x,
            y: self.y - min_y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.len(), 14);
    }
}
