use std::collections::{HashMap, HashSet};

use crate::challenges::coordinates::get_possible_neighbours;

use super::cube::{new_cube, Cube};
use super::{
    coordinates::{new_coordinates, Coordinates},
    read_lines::read_lines,
};

pub fn solve_first_challenge(cubes: &HashMap<Coordinates, Cube>) {
    let total: u32 = cubes
        .iter()
        .map(|(_, v)| 6 - v.neighbours.len())
        .map(|v| u32::try_from(v).unwrap())
        .sum();
    println!("Total: {}", total);
}

pub fn build_cube_map_from_lines(filename: &str) -> HashMap<Coordinates, Cube> {
    let mut cubes: HashMap<Coordinates, Cube> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for (_, line) in lines.flatten().enumerate() {
            if !line.is_empty() {
                let line_coords: Vec<&str> = line.split(',').collect();
                let x: i32 = line_coords[0].parse().unwrap();
                let y: i32 = line_coords[1].parse().unwrap();
                let z: i32 = line_coords[2].parse().unwrap();
                let coords = new_coordinates(x, y, z);
                cubes.insert(coords, new_cube(coords));
            }
        }
    }

    fill_neighbours(&mut cubes);

    //println!("Cubes: {:?}", cubes);
    cubes
}

fn fill_neighbours(cubes: &mut HashMap<Coordinates, Cube>) {
    let mut set: HashSet<Coordinates> = HashSet::new();
    cubes.iter().for_each(|e| {
        set.insert(*e.0);
    });

    cubes.iter_mut().for_each(|entry| {
        for p in get_possible_neighbours(*entry.0) {
            if set.contains(&p) {
                entry.1.neighbours.insert(p);
            }
        }
    });
}
