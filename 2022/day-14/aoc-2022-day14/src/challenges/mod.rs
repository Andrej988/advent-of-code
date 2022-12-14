use std::collections::HashMap;

use self::coordinates::Coordinates;

mod coordinates;
mod direction;
mod first_challenge;
mod load_data;
mod read_lines;

pub fn build_map_from_file(filename: &str) -> HashMap<Coordinates, char> {
    load_data::build_map_from_file(filename)
}

#[allow(dead_code)]
pub fn print_map(map: &HashMap<Coordinates, char>) {
    load_data::print(map);
}

pub fn solve_first_challenge(map: &mut HashMap<Coordinates, char>) {
    first_challenge::solve_challenge(map);
}
