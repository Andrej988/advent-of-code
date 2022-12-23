use std::collections::HashMap;

use self::coordinates::Coordinates;

mod challenge;
mod coordinates;
mod direction;
mod parse_lines;
mod quadrant;
mod read_lines;
mod state;

pub fn parse_input(filename: &str) -> (HashMap<Coordinates, String>, Vec<String>, Coordinates) {
    parse_lines::parse_lines(filename)
}

pub fn solve_first_challenge(
    coords: &HashMap<Coordinates, String>,
    instructions: &[String],
    starting_coords: &Coordinates,
) {
    challenge::solve_first_challenge(coords, instructions, starting_coords)
}

pub fn solve_second_challenge(
    coords: &HashMap<Coordinates, String>,
    instructions: &[String],
    starting_coords: &Coordinates,
) {
    challenge::solve_second_challenge(coords, instructions, starting_coords)
}
