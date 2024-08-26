use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            MirrorType::Backslash => match dir {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Beam {
    loc: (isize, isize),
    dir: Direction,
}
impl Beam {
    fn step(&self) -> Beam {
        let (dx, dy) = self.dir.to_tuple();
        let loc = (self.loc.0 + dx, self.loc.1 + dy);
        let dir = self.dir;
        Beam { loc, dir }
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

    fn in_bounds(&self, height: usize, width: usize) -> bool {
        let (x, y) = self.loc;
        let out = x < 0 || x >= width as isize || y < 0 || y >= height as isize;
        !out
    }

    fn default_beam() -> Beam {
        Beam {
            loc: (0, 0),
            dir: Direction::Right,
        }
    }
}

#[derive(Clone)]
struct Contraption {
    height: usize,
    width: usize,
    beams: Vec<Beam>,
    mirrors: HashMap<(isize, isize), MirrorType>,
    energized: HashSet<Beam>,
}
impl Contraption {
    fn new(height: usize, width: usize, mirrors: HashMap<(isize, isize), MirrorType>) -> Self {
        let beam = Beam::default_beam();
        let beams = if let Some(mirror) = mirrors.get(&beam.loc) {
            beam.reflect(mirror)
        } else {
            vec![beam]
        };

        Contraption {
            height,
            width,
            beams,
            mirrors,
            energized: HashSet::new(),
        }
    }

    fn reflect_beam(&self, beam: Beam) -> Vec<Beam> {
        if let Some(mirror) = self.mirrors.get(&beam.loc) {
            beam.reflect(mirror)
        } else {
            vec![beam]
        }
    }

    fn step(&mut self) {
        self.beams.iter().for_each(|b| {
            self.energized.insert(*b);
        });

        self.beams = self
            .beams
            .iter()
            .map(|&b| b.step())
            .map(|b| self.reflect_beam(b))
            .flatten()
            .filter(|beam| beam.in_bounds(self.height, self.width))
            .filter(|beam| !self.energized.contains(beam))
            .collect();
    }

    fn energized_tiles(&mut self) -> Option<u32> {
        while !self.beams.is_empty() {
            self.step();
        }
        Some(self.energized.iter().map(|b| b.loc).unique().count() as u32)
    }

    fn init_beams(&self, loc: (isize, isize)) -> Vec<Beam> {
        let beams = self.init_beam(loc);
        beams
            .iter()
            .map(|beam| self.reflect_beam(*beam))
            .flatten()
            .collect()
    }

    fn init_beam(&self, loc: (isize, isize)) -> Vec<Beam> {
        let (width, height) = (self.width as isize - 1, self.height as isize - 1);
        match loc {
            l if l == (0, 0) => vec![
                Beam {
                    loc,
                    dir: Direction::Right,
                },
                Beam {
                    loc,
                    dir: Direction::Down,
                },
            ],
            l if l == (height, 0) => vec![
                Beam {
                    loc,
                    dir: Direction::Left,
                },
                Beam {
                    loc,
                    dir: Direction::Down,
                },
            ],
            l if l == (0, height) => vec![
                Beam {
                    loc,
                    dir: Direction::Right,
                },
                Beam {
                    loc,
                    dir: Direction::Up,
                },
            ],
            l if l == (height, width) => vec![
                Beam {
                    loc,
                    dir: Direction::Left,
                },
                Beam {
                    loc,
                    dir: Direction::Up,
                },
            ],
            (0, _) => vec![Beam {
                loc,
                dir: Direction::Right,
            }],
            (_, 0) => vec![Beam {
                loc,
                dir: Direction::Down,
            }],
            l if l.0 == width => vec![Beam {
                loc,
                dir: Direction::Up,
            }],
            l if l.1 == height => vec![Beam {
                loc,
                dir: Direction::Left,
            }],
            (_, _) => vec![],
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut contraption = parse_input(input);
    // while contraption.beams.len() > 0 {
    contraption.energized_tiles()
}

pub fn part_two(input: &str) -> Option<u32> {
    let contraption = parse_input(input);
    let (w, h) = (contraption.width, contraption.height);
    let starts: Vec<_> = (0..w)
        .cartesian_product(0..h)
        .map(|(x, y)| (x, y))
        .filter(|(x, y)| *x == 0 || *x == w - 1 || *y == 0 || *y == h - 1)
        .collect();

    starts
        .par_iter()
        .map(|(x, y)| {
            let beams = contraption.init_beams((*x as isize, *y as isize));
            beams
                .iter()
                .map(|beam| {
                    let mut contraption = contraption.clone();
                    contraption.beams = vec![*beam];
                    contraption.energized = HashSet::new();
                    contraption.energized_tiles().unwrap()
                })
                .max()
        })
        .flatten()
        .max()
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
        assert_eq!(result, Some(51));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.mirrors.len(), 23);
    }
}
