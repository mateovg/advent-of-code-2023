use combinations::Combinations;
use std::ops::Range;

advent_of_code::solution!(11);

#[derive(Debug)]
struct Universe {
    map: Vec<Vec<char>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    expansion: i32,
}
impl Universe {
    fn new(map: Vec<Vec<char>>) -> Universe {
        Universe {
            empty_rows: Universe::find_empty_rows(&map),
            empty_cols: Universe::find_empty_cols(&map),
            map,
            expansion: 2,
        }
    }

    fn find_empty_rows(map: &Vec<Vec<char>>) -> Vec<usize> {
        map.iter()
            .enumerate()
            .map(|(i, l)| (i, l.iter().any(|x| x == &'#')))
            .filter(|(_, has_galaxy)| !has_galaxy)
            .map(|(i, _)| i)
            .collect()
    }

    fn find_empty_cols(map: &Vec<Vec<char>>) -> Vec<usize> {
        let mut empty_cols = Vec::new();
        for col in 0..map[0].len() {
            let column: Vec<char> = map.iter().map(|l| l[col]).filter(|x| x == &'#').collect();
            if column.len() == 0 {
                empty_cols.push(col);
            }
        }
        empty_cols
    }

    fn find_galaxies(&self) -> Vec<Galaxy> {
        self.map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, c)| Galaxy::new(x, y, c))
                    .collect::<Vec<Galaxy>>()
            })
            .flatten()
            .collect()
    }

    fn find_galaxy_pairs(&self) -> Vec<Vec<Galaxy>> {
        Combinations::new(self.find_galaxies(), 2).collect()
    }

    fn find_galaxy_distance(&self, start: Galaxy, end: Galaxy) -> u64 {
        let (x_range, y_range) = start.distance_ranges(&end);
        let x_distance: u64 = x_range
            .map(|x| {
                if self.empty_cols.contains(&x) {
                    self.expansion as u64
                } else {
                    1
                }
            })
            .sum();
        let y_distance: u64 = y_range
            .map(|y| {
                if self.empty_rows.contains(&y) {
                    self.expansion as u64
                } else {
                    1
                }
            })
            .sum();

        x_distance + y_distance
    }

    fn find_galaxy_distances(&self) -> u64 {
        self.find_galaxy_pairs()
            .iter()
            .map(|pair| self.find_galaxy_distance(pair[0], pair[1]))
            .sum()
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialEq, PartialOrd, Eq)]
struct Galaxy {
    x: usize,
    y: usize,
}
impl Galaxy {
    fn new(x: usize, y: usize, symb: &char) -> Option<Galaxy> {
        if *symb != '#' {
            return None;
        }
        Some(Galaxy { x, y })
    }

    fn distance_ranges(&self, other: &Galaxy) -> (Range<usize>, Range<usize>) {
        let x_range = if self.x > other.x {
            other.x..self.x
        } else {
            self.x..other.x
        };
        let y_range = if self.y > other.y {
            other.y..self.y
        } else {
            self.y..other.y
        };
        (x_range, y_range)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let universe = parse_input(input);
    Some(universe.find_galaxy_distances())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut universe = parse_input(input);
    universe.expansion = 1_000_000;
    Some(universe.find_galaxy_distances())
}

fn parse_input(input: &str) -> Universe {
    let map = input.lines().map(|l| l.chars().collect()).collect();
    Universe::new(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.map.len(), 10);
    }

    #[test]
    fn test_find_empty_rows() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.empty_rows, vec![3, 7]);
    }

    #[test]
    fn test_find_empty_cols() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.empty_cols, vec![2, 5, 8]);
    }

    #[test]
    fn test_find_galaxies() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.find_galaxies().len(), 9);
    }

    #[test]
    fn test_find_galaxies_pairs() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.find_galaxy_pairs().len(), 36);
    }

    #[test]
    fn test_galaxy_range() {
        let a = Galaxy { x: 0, y: 10 };
        let b = Galaxy { x: 0, y: 0 };
        assert_eq!(a.distance_ranges(&b), (0..0, 0..10));
        let a = Galaxy { x: 10, y: 10 };
        let b = Galaxy { x: 0, y: 0 };
        assert_eq!(a.distance_ranges(&b), (0..10, 0..10));
        let a = Galaxy { x: 10, y: 10 };
        let b = Galaxy { x: 200, y: 0 };
        assert_eq!(a.distance_ranges(&b), (10..200, 0..10));
    }

    #[test]
    fn test_galaxy_distnace() {
        let universe = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let galaxies = universe.find_galaxies();
        let result = universe.find_galaxy_distance(galaxies[0], galaxies[6]);
        assert_eq!(result, 15);
        let result = universe.find_galaxy_distance(galaxies[2], galaxies[5]);
        dbg!(&galaxies);
        assert_eq!(result, 17);
    }
}
