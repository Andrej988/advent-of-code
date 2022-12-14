use self::Direction::*;
use std::slice::Iter;

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    DownAndLeft,
    DownAndRight,
}

impl Direction {
    #[allow(dead_code)]
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 6] = [Up, Down, Right, Left, DownAndLeft, DownAndRight];
        DIRECTIONS.iter()
    }
}
