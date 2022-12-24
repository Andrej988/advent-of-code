use self::Direction::*;
use std::slice::Iter;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Left, Right, Up, Down];
        DIRECTIONS.iter()
    }
}

pub fn oposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    }
}
