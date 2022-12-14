use std::collections::HashMap;

use crate::challenges::coordinates::move_towards;

use super::{
    coordinates::{new_coordinates, Coordinates},
    read_lines::read_lines,
};

pub fn build_map_from_file(filename: &str) -> HashMap<Coordinates, char> {
    let mut map: HashMap<Coordinates, char> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for (_, line) in lines.flatten().enumerate() {
            if !line.is_empty() {
                let coords: Vec<&str> = line.split("->").collect();

                for i in 0..coords.len() - 1 {
                    let from: Vec<&str> = coords[i].trim().split(',').collect();
                    let to: Vec<&str> = coords[i + 1].trim().split(',').collect();

                    let mut coords_from: Coordinates =
                        new_coordinates(to_usize(from[1]), to_usize(from[0]));
                    let coords_to: Coordinates = new_coordinates(to_usize(to[1]), to_usize(to[0]));

                    loop {
                        map.insert(coords_from, '#');
                        coords_from = move_towards(coords_from, coords_to);

                        if coords_from == coords_to {
                            map.insert(coords_from, '#');
                            break;
                        }
                    }
                }
            }
        }
    }
    map
}

fn to_usize(x: &str) -> usize {
    let a: usize = x.parse().unwrap();
    a
}

#[allow(dead_code)]
pub fn print(map: &HashMap<Coordinates, char>) {
    for (key, value) in map {
        println!("Key {} Value: {}", key, value);
    }
}

pub fn find_biggest_x(map: &HashMap<Coordinates, char>) -> usize {
    map.iter()
        .filter(|entry| *entry.1 == '#')
        .map(|entry| entry.0.x)
        .max()
        .unwrap()
}

pub fn is_coordinate_reachable(coords: &Coordinates, map: &HashMap<Coordinates, char>) -> bool {
    !map.contains_key(coords)
}

pub fn count_chars(map: &HashMap<Coordinates, char>, c: char) -> usize {
    map.iter().filter(|entry| *entry.1 == c).count()
}
