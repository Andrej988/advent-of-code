use std::collections::HashMap;

use self::{coordinates::Coordinates, cube::Cube};

mod challenge;
mod coordinates;
mod cube;
mod read_lines;

pub fn build_cube_map(filename: &str) -> HashMap<Coordinates, Cube> {
    challenge::build_cube_map_from_lines(filename)
}

pub fn solve_first_challenge(cubes: &HashMap<Coordinates, Cube>) {
    challenge::solve_first_challenge(cubes);
}

pub fn solve_second_challenge(cubes: &HashMap<Coordinates, Cube>) {
    challenge::solve_second_challenge_waterfill(cubes);
}
