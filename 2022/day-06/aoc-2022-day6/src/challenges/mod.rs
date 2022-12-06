mod challenge;
mod read_lines;

use crate::challenges::challenge::solve_challenge;

pub fn first_challenge(filename: &str) {
    solve_challenge(filename, &4);
}

pub fn second_challenge(filename: &str) {
    solve_challenge(filename, &14);
}
