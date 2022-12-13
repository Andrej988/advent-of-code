use std::collections::HashSet;

use crate::challenges::coordinates::new_coordinates;

use super::coordinates::Coordinates;

pub fn elevation_to_set(elevation: &[Vec<u8>]) -> HashSet<Coordinates> {
    let mut vector: HashSet<Coordinates> = HashSet::new();
    for (x, row) in elevation.iter().enumerate() {
        for y in 0..row.len() {
            vector.insert(new_coordinates(x, y));
        }
    }
    vector
}
