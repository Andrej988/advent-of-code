use std::{
    cmp,
    collections::{HashMap, HashSet},
    fmt,
};

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

pub fn get_max_sides(cubes: &HashMap<Coordinates, Cube>) -> (i32, i32, i32) {
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut max_z: i32 = 0;

    for (k, _) in cubes.iter() {
        max_x = cmp::max(max_x, k.x);
        max_y = cmp::max(max_y, k.y);
        max_z = cmp::max(max_z, k.z);
    }

    (max_x, max_y, max_z)
}

pub fn get_min_sides(cubes: &HashMap<Coordinates, Cube>) -> (i32, i32, i32) {
    let mut min_x: i32 = 100000000;
    let mut min_y: i32 = 100000000;
    let mut min_z: i32 = 100000000;

    for (k, _) in cubes.iter() {
        min_x = cmp::min(min_x, k.x);
        min_y = cmp::min(min_y, k.y);
        min_z = cmp::min(min_z, k.z);
    }

    (min_x, min_y, min_z)
}
