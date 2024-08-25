advent_of_code::solution!(10);

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Debug)]
struct Sketch {
    grid: Vec<Vec<PipeType>>,
    width: usize,
    height: usize,
}
impl Sketch {
    fn find_start(&self) -> (usize, usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == PipeType::Start {
                    return (x, y);
                }
            }
        }
        (0, 0)
    }

    fn follow_path(&self) -> Vec<(usize, usize)> {
        let mut path = vec![];
        let mut current = self.find_start();

        path
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PipeType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    None,
}
impl PipeType {
    fn from_char(c: char) -> PipeType {
        match c {
            '|' => PipeType::NorthSouth,
            '-' => PipeType::EastWest,
            'L' => PipeType::NorthEast,
            'J' => PipeType::NorthWest,
            '7' => PipeType::SouthWest,
            'F' => PipeType::SouthEast,
            'S' => PipeType::Start,
            _ => PipeType::None,
        }
    }

    // Returns if you can enter the pipe from a given direction.
    fn valid(&self, direction: (isize, isize)) -> bool {
        match direction {
            (0, -1) => matches!(
                self,
                PipeType::NorthSouth | PipeType::NorthEast | PipeType::NorthWest
            ),
            (0, 1) => matches!(
                self,
                PipeType::NorthSouth | PipeType::SouthEast | PipeType::SouthWest
            ),
            (-1, 0) => matches!(
                self,
                PipeType::EastWest | PipeType::NorthEast | PipeType::SouthEast
            ),
            (1, 0) => matches!(
                self,
                PipeType::EastWest | PipeType::NorthWest | PipeType::SouthWest
            ),
            _ => false,
        }
    }

    // Returns where you come out of after entering a pipe from a given direction
    fn next(&self, direction: (isize, isize)) -> Option<(isize, isize)> {
        if !self.valid(direction) {
            return None;
        }
        match direction {
            (0, -1) => match self {
                PipeType::NorthSouth => Some((0, -1)),
                PipeType::NorthEast => Some((1, 0)),
                PipeType::NorthWest => Some((-1, 0)),
                _ => None,
            },
            (0, 1) => match self {
                PipeType::NorthSouth => Some((0, 1)),
                PipeType::SouthEast => Some((1, 0)),
                PipeType::SouthWest => Some((-1, 0)),
                _ => None,
            },
            (-1, 0) => match self {
                PipeType::EastWest => Some((-1, 0)),
                PipeType::NorthEast => Some((0, -1)),
                PipeType::SouthEast => Some((0, 1)),
                _ => None,
            },
            (1, 0) => match self {
                PipeType::EastWest => Some((1, 0)),
                PipeType::NorthWest => Some((0, -1)),
                PipeType::SouthWest => Some((0, 1)),
                _ => None,
            },
            _ => None,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sketch = parse_input(input);
    // dbg!(&sketch);
    let path = sketch.follow_path();
    dbg!(&path);
    Some(path.len() as u32 / 2);
    todo!();
}

pub fn part_two(input: &str) -> Option<u32> {
    todo!()
}

fn parse_input(input: &str) -> Sketch {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| PipeType::from_char(c))
                .collect::<Vec<PipeType>>()
        })
        .collect::<Vec<Vec<PipeType>>>();

    Sketch {
        width: grid[0].len(),
        height: grid.len(),
        grid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.grid.len(), 5);
    }

    #[test]
    fn test_find_start() {
        let sketch = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = sketch.find_start();
        assert_eq!(result, (0, 2));
    }

    #[test]
    fn test_valid_pipe() {
        let pipe = PipeType::NorthSouth;
        assert_eq!(pipe.valid((0, -1)), true);
        assert_eq!(pipe.valid((0, 1)), true);
        assert_eq!(pipe.valid((-1, 0)), false);
        assert_eq!(pipe.valid((1, 0)), false);
    }
}
