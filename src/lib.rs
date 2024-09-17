pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}
