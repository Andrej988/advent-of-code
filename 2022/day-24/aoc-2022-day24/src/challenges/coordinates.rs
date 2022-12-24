use std::fmt;

use super::direction::Direction;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {} Y: {}", self.x, self.y)
    }
}

pub fn new_coordinates(x: usize, y: usize) -> Coordinates {
    Coordinates { x, y }
}

pub trait Move {
    fn peek_right(&self) -> Coordinates;
    fn peek_left(&self) -> Coordinates;
    fn peek_up(&self) -> Coordinates;
    fn peek_down(&self) -> Coordinates;
    fn peek(&self, direction: Direction) -> Coordinates;
}

impl Move for Coordinates {
    fn peek_right(&self) -> Coordinates {
        new_coordinates(self.x, self.y + 1)
    }

    fn peek_left(&self) -> Coordinates {
        new_coordinates(self.x, self.y - 1)
    }

    fn peek_up(&self) -> Coordinates {
        new_coordinates(self.x - 1, self.y)
    }

    fn peek_down(&self) -> Coordinates {
        new_coordinates(self.x + 1, self.y)
    }

    fn peek(&self, direction: Direction) -> Coordinates {
        match direction {
            Direction::Left => self.peek_left(),
            Direction::Right => self.peek_right(),
            Direction::Up => self.peek_up(),
            Direction::Down => self.peek_down(),
        }
    }
}
