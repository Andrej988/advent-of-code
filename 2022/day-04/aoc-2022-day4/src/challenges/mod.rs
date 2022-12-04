mod first_challenge;
mod second_challenge;
mod read_lines;

use crate::challenges::first_challenge::first_challenge as challenge1;
use crate::challenges::second_challenge::second_challenge as challenge2;

pub fn first_challenge() {
    challenge1();
}

pub fn second_challenge() {
    challenge2();
}