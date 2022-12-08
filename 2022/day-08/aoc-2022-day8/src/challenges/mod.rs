mod first_challenge;
mod load_data;
mod read_lines;
mod second_challenge;

use crate::challenges::first_challenge::solve_challenge as challenge1;
use crate::challenges::second_challenge::solve_challenge as challenge2;

pub fn first_challenge(filename: &str) {
    challenge1(filename);
}

pub fn second_challenge(filename: &str) {
    challenge2(filename);
}
