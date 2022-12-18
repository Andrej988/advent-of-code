use std::{collections::HashSet, fmt};

use super::coordinates::Coordinates;

#[derive(Debug)]
pub struct Cube {
    pub coords: Coordinates,
    pub neighbours: HashSet<Coordinates>,
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Cube coords: X: {} Y: {}, Z: {}",
            self.coords.x, self.coords.y, self.coords.z
        )
    }
}

pub fn new_cube(coords: Coordinates) -> Cube {
    Cube {
        coords,
        neighbours: HashSet::new(),
    }
}
