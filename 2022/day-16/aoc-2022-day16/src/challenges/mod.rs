use std::collections::HashMap;

use self::valves::Valve;

mod challenge;
mod read_lines;
mod valves;

pub fn build_valves_map(filename: &str) -> HashMap<String, Valve> {
    valves::build_valves_map(filename)
}

pub fn solve_first_challenge(valves: &HashMap<String, Valve>) {
    challenge::solve_first_challenge(valves);
}
