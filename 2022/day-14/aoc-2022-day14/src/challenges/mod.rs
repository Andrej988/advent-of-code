use std::collections::HashMap;

use self::coordinates::Coordinates;

mod coordinates;
mod direction;
mod first_challenge;
mod map;
mod read_lines;
mod second_challenge;

pub fn build_map_from_file(filename: &str) -> HashMap<Coordinates, char> {
    map::build_map_from_file(filename)
}

#[allow(dead_code)]
pub fn print_map(map: &HashMap<Coordinates, char>) {
    map::print(map);
}

pub fn solve_first_challenge(map: &mut HashMap<Coordinates, char>) {
    first_challenge::solve_challenge(map);
}

pub fn solve_second_challenge(map: &mut HashMap<Coordinates, char>) {
    second_challenge::solve_challenge(map);
}
