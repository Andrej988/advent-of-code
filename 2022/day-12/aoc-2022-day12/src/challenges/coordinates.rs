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

pub fn perform_move(current_coordinates: &Coordinates, direction: &Direction) -> Coordinates {
    match direction {
        Direction::Down => new_coordinates(current_coordinates.x + 1, current_coordinates.y),
        Direction::Up => new_coordinates(current_coordinates.x - 1, current_coordinates.y),
        Direction::Right => new_coordinates(current_coordinates.x, current_coordinates.y + 1),
        Direction::Left => new_coordinates(current_coordinates.x, current_coordinates.y - 1),
    }
}

pub fn get_coordinates_key(current_coordinates: &Coordinates) -> String {
    format!("{}:{}", current_coordinates.x, current_coordinates.y)
}

pub fn is_possible_move_direction(
    elevation: &Vec<Vec<u8>>,
    coordinates: Coordinates,
    direction: &Direction,
) -> bool {
    match direction {
        Direction::Down => coordinates.x < elevation.len() - 1,
        Direction::Up => coordinates.x > 0,
        Direction::Right => coordinates.y < elevation[0].len() - 1,
        Direction::Left => coordinates.y > 0,
    }
}

pub fn is_allowed_to_move(
    elevation: &[Vec<u8>],
    current_coords: Coordinates,
    new_coords: Coordinates,
) -> bool {
    let current_coords_val = elevation[current_coords.x][current_coords.y];
    let new_coords_val = elevation[new_coords.x][new_coords.y];

    i8::try_from(new_coords_val).unwrap() - i8::try_from(current_coords_val).unwrap() <= 1
}

pub fn is_allowed_to_move_reverse(
    elevation: &[Vec<u8>],
    current_coords: Coordinates,
    new_coords: Coordinates,
) -> bool {
    let current_coords_val = elevation[current_coords.x][current_coords.y];
    let new_coords_val = elevation[new_coords.x][new_coords.y];

    i8::try_from(current_coords_val).unwrap() - i8::try_from(new_coords_val).unwrap() <= 1
}
