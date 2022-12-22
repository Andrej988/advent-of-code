use super::{
    coordinates::{Coordinates, Move},
    direction::Direction,
};

pub struct State {
    pub current_coords: Coordinates,
    pub current_direction: Direction,
}

pub fn new_state(current_coords: Coordinates, current_direction: Direction) -> State {
    State {
        current_coords,
        current_direction,
    }
}

pub fn build_starting_state(starting_coords: Coordinates) -> State {
    new_state(starting_coords, Direction::Right)
}

pub trait Rotate {
    fn rotate(&mut self, direction: Direction);
}

impl Rotate for State {
    fn rotate(&mut self, direction: Direction) {
        match self.current_direction {
            Direction::Up => {
                self.current_direction = if direction == Direction::Left {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            Direction::Left => {
                self.current_direction = if direction == Direction::Left {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
            Direction::Right => {
                self.current_direction = if direction == Direction::Left {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
            Direction::Down => {
                self.current_direction = if direction == Direction::Left {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
        }
    }
}

pub trait MoveState {
    fn peek(&self) -> Coordinates;
    fn move_to_new_coords(&mut self, new_coordinates: Coordinates);
}

impl MoveState for State {
    fn peek(&self) -> Coordinates {
        self.current_coords.peek(self.current_direction)
    }

    fn move_to_new_coords(&mut self, new_coordinates: Coordinates) {
        self.current_coords = new_coordinates;
    }
}
