use self::Direction::*;
use std::slice::Iter;

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Up, Down, Right, Left];
        DIRECTIONS.iter()
    }
}
