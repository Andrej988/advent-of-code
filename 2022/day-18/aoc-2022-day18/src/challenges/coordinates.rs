use std::{collections::HashSet, fmt};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X: {} Y: {}, Z: {}", self.x, self.y, self.z)
    }
}

pub fn new_coordinates(x: i32, y: i32, z: i32) -> Coordinates {
    Coordinates { x, y, z }
}

pub fn get_possible_neighbours(coords: Coordinates) -> HashSet<Coordinates> {
    let mut possible_neighbours: HashSet<Coordinates> = HashSet::new();
    possible_neighbours.insert(new_coordinates(coords.x - 1, coords.y, coords.z));
    possible_neighbours.insert(new_coordinates(coords.x + 1, coords.y, coords.z));
    possible_neighbours.insert(new_coordinates(coords.x, coords.y - 1, coords.z));
    possible_neighbours.insert(new_coordinates(coords.x, coords.y + 1, coords.z));
    possible_neighbours.insert(new_coordinates(coords.x, coords.y, coords.z - 1));
    possible_neighbours.insert(new_coordinates(coords.x, coords.y, coords.z + 1));
    possible_neighbours
}
