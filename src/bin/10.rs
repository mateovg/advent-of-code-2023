advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Sketch {
    grid: Vec<Vec<PipeType>>,
    width: usize,
    height: usize,
}
impl Sketch {
    fn find_start(&self) -> Point {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == PipeType::Start {
                    return Point { x, y };
                }
            }
        }
        Point { x: 0, y: 0 }
    }

    fn follow_path(&self) -> Vec<(usize, usize)> {
        let mut path = vec![];
        let current = self.find_start();

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

    fn neighbor(self, Point { x, y }: Point) -> Vec<Point> {
        match self {
            PipeType::NorthSouth => vec![Point { x, y: y + 1 }, Point { x, y: y - 1 }],
            PipeType::NorthEast => vec![Point { x, y: y + 1 }, Point { x: x + 1, y }],
            PipeType::NorthWest => vec![Point { x, y: y + 1 }, Point { x: x - 1, y }],
            PipeType::SouthEast => vec![Point { x, y: y - 1 }, Point { x: x + 1, y }],
            PipeType::SouthWest => vec![Point { x, y: y - 1 }, Point { x: x - 1, y }],
            PipeType::EastWest => vec![Point { x: x + 1, y }, Point { x: x + 1, y }],
            PipeType::Start => vec![
                Point { x, y: y + 1 },
                Point { x, y: y - 1 },
                Point { x: x + 1, y },
                Point { x: x + 1, y },
            ],
            PipeType::None => vec![],
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let sketch = parse_input(input);
    // dbg!(&sketch);
    let path = sketch.follow_path();
    dbg!(&path);
    Some(path.len() as usize / 2);
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
        assert_eq!(result, Point { x: 0, y: 2 });
    }
}
