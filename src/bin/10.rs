advent_of_code::solution!(10);

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
}

#[derive(Debug, PartialEq, Eq)]
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
}

pub fn part_one(input: &str) -> Option<u32> {
    let sketch = parse_input(input);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
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
        assert_eq!(result, Some(8));
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
}
