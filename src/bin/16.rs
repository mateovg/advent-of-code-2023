use std::collections::{HashMap, HashSet};

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn to_tuple(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MirrorType {
    Vertical,
    Horizontal,
    Slash,
    Backslash,
}
impl MirrorType {
    fn from_char(c: char) -> Option<MirrorType> {
        match c {
            '|' => Some(MirrorType::Vertical),
            '-' => Some(MirrorType::Horizontal),
            '/' => Some(MirrorType::Slash),
            '\\' => Some(MirrorType::Backslash),
            _ => None,
        }
    }

    fn reflect(&self, dir: Direction) -> Vec<Direction> {
        match self {
            MirrorType::Vertical => match dir {
                Direction::Up | Direction::Down => vec![dir],
                _ => vec![Direction::Up, Direction::Down],
            },
            MirrorType::Horizontal => match dir {
                Direction::Left | Direction::Right => vec![dir],
                _ => vec![Direction::Left, Direction::Right],
            },
            MirrorType::Slash => match dir {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            MirrorType::Backslash => match dir {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    loc: (isize, isize),
    dir: Direction,
}
impl Beam {
    fn step(&self) -> Beam {
        let (dx, dy) = self.dir.to_tuple();
        Beam {
            loc: (self.loc.0 + dx, self.loc.1 + dy),
            dir: self.dir,
        }
    }

    fn reflect(&self, mirror: &MirrorType) -> Vec<Beam> {
        mirror
            .reflect(self.dir)
            .iter()
            .map(|dir| Beam {
                loc: self.loc,
                dir: dir.clone(),
            })
            .collect()
    }
}

struct Contraption {
    height: usize,
    width: usize,
    beams: Vec<Beam>,
    mirrors: HashMap<(isize, isize), MirrorType>,
    energized: HashSet<(isize, isize)>,
}
impl Contraption {
    fn new(height: usize, width: usize, mirrors: HashMap<(isize, isize), MirrorType>) -> Self {
        let beams = vec![Beam {
            loc: (0, 0),
            dir: Direction::Right,
        }];
        Contraption {
            height,
            width,
            beams,
            mirrors,
            energized: HashSet::new(),
        }
    }

    fn step(&mut self) -> () {
        dbg!(&self.beams);
        self.beams
            .iter_mut()
            .filter(|beam| {
                beam.loc.0 >= 0
                    && beam.loc.0 < self.width as isize
                    && beam.loc.1 >= 0
                    && beam.loc.1 < self.height as isize
            })
            .map(|beam| {
                if let Some(mirror) = self.mirrors.get(&beam.loc) {
                    beam.reflect(mirror)
                } else {
                    vec![*beam]
                }
            })
            .map(|mut beam| beam.iter_mut().map(|b| b.step()).collect::<Vec<Beam>>())
            .flatten()
            .for_each(|beam: Beam| {
                self.energized.insert(beam.loc);
            });
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut contraption = parse_input(input);
    for _ in 0..10 {
        contraption.step();
    }
    Some(contraption.energized.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Contraption {
    let mirrors = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), MirrorType::from_char(c)))
                .filter(|(_, m)| m.is_some())
                .map(|(loc, mirror)| (loc, mirror.unwrap()))
                .collect::<HashMap<(isize, isize), MirrorType>>()
        })
        .flatten()
        .collect();
    let (width, height) = (input.lines().next().unwrap().len(), input.lines().count());
    Contraption::new(height, width, mirrors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.mirrors.len(), 23);
    }
}
