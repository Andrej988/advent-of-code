mod challenge;
mod read_lines;

use crate::challenges::challenge::solve_challenge as challenge1;

pub fn first_challenge(filename: &str) {
    challenge1(filename);
}