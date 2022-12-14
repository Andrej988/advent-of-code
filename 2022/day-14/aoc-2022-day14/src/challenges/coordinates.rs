use std::fmt;

use super::direction::Direction;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
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

pub fn move_towards(from: Coordinates, to: Coordinates) -> Coordinates {
    if from != to {
        if from.x < to.x {
            return perform_move(&from, &Direction::Down);
        } else if from.x > to.x {
            return perform_move(&from, &Direction::Up);
        } else if from.y < to.y {
            return perform_move(&from, &Direction::Right);
        } else if from.y > to.y {
            return perform_move(&from, &Direction::Left);
        }
    }
    new_coordinates(from.x, from.y)
}

pub fn perform_move(current_coordinates: &Coordinates, direction: &Direction) -> Coordinates {
    match direction {
        Direction::Down => new_coordinates(current_coordinates.x + 1, current_coordinates.y),
        Direction::Up => new_coordinates(current_coordinates.x - 1, current_coordinates.y),
        Direction::Right => new_coordinates(current_coordinates.x, current_coordinates.y + 1),
        Direction::Left => new_coordinates(current_coordinates.x, current_coordinates.y - 1),
        Direction::DownAndLeft => {
            new_coordinates(current_coordinates.x + 1, current_coordinates.y - 1)
        }
        Direction::DownAndRight => {
            new_coordinates(current_coordinates.x + 1, current_coordinates.y + 1)
        }
    }
}
