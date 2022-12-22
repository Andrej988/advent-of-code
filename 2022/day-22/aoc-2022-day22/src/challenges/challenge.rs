use std::collections::HashMap;

use super::{
    coordinates::{new_coordinates, Coordinates, Move},
    direction::{oposite_direction, Direction},
    state::{build_starting_state, MoveState, Rotate, State},
};

pub fn solve_first_challenge(
    coords: &HashMap<Coordinates, String>,
    instructions: &[String],
    starting_coords: &Coordinates,
) {
    let mut state: State = build_starting_state(*starting_coords);

    for (_, instruction) in instructions.iter().enumerate() {
        if is_numeric_string(instruction) {
            let num_of_moves = instruction.parse::<i64>().unwrap();
            for _ in 0..num_of_moves {
                let mut next_coords = state.peek();
                if !coords.contains_key(&next_coords) {
                    let mut curr_coords = state
                        .current_coords
                        .peek(oposite_direction(state.current_direction));
                    while coords.contains_key(&curr_coords) {
                        next_coords = new_coordinates(curr_coords.x, curr_coords.y);
                        curr_coords = curr_coords.peek(oposite_direction(state.current_direction));
                    }
                }

                if coords.get(&next_coords).unwrap().trim().starts_with('.') {
                    state.move_to_new_coords(next_coords);
                }
            }
        } else {
            let rotate_direction = if instruction.trim().to_uppercase().starts_with('R') {
                Direction::Right
            } else {
                Direction::Left
            };
            state.rotate(rotate_direction);
        }
    }

    println!("Current coords: {}", state.current_coords);
    let result = 1000 * state.current_coords.x
        + 4 * state.current_coords.y
        + get_value_for_direction(state.current_direction);
    println!("Result: {}", result);
}

fn is_numeric_string(value: &str) -> bool {
    let test = value.parse::<i64>();
    match test {
        Ok(_ok) => true,
        Err(_e) => false,
    }
}

fn get_value_for_direction(direction: Direction) -> i64 {
    match direction {
        Direction::Left => 2,
        Direction::Right => 0,
        Direction::Up => 3,
        Direction::Down => 1,
    }
}
