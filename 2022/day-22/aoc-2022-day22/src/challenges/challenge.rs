/*--- Day 22: Monkey Map ---
The monkeys take you on a surprisingly easy trail through the jungle. They're even going in roughly the right direction according to your handheld device's Grove Positioning System.

As you walk, the monkeys explain that the grove is protected by a force field. To pass through the force field, you have to enter a password; doing so involves tracing a specific path on a strangely-shaped board.

At least, you're pretty sure that's what you have to do; the elephants aren't exactly fluent in monkey.

The monkeys give you notes that they took when they last saw the password entered (your puzzle input).

For example:

        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
The first half of the monkeys' notes is a map of the board. It is comprised of a set of open tiles (on which you can move, drawn .) and solid walls (tiles which you cannot enter, drawn #).

The second half is a description of the path you must follow. It consists of alternating numbers and letters:

A number indicates the number of tiles to move in the direction you are facing. If you run into a wall, you stop moving forward and continue with the next instruction.
A letter indicates whether to turn 90 degrees clockwise (R) or counterclockwise (L). Turning happens in-place; it does not change your current tile.
So, a path like 10R5 means "go forward 10 tiles, then turn clockwise 90 degrees, then go forward 5 tiles".

You begin the path in the leftmost open tile of the top row of tiles. Initially, you are facing to the right (from the perspective of how the map is drawn).

If a movement instruction would take you off of the map, you wrap around to the other side of the board. In other words, if your next tile is off of the board, you should instead look in the direction opposite of your current facing as far as you can until you find the opposite edge of the board, then reappear there.

For example, if you are at A and facing to the right, the tile in front of you is marked B; if you are at C and facing down, the tile in front of you is marked D:

        ...#
        .#..
        #...
        ....
...#.D.....#
........#...
B.#....#...A
.....C....#.
        ...#....
        .....#..
        .#......
        ......#.
It is possible for the next tile (after wrapping around) to be a wall; this still counts as there being a wall in front of you, and so movement stops before you actually wrap to the other side of the board.

By drawing the last facing you had with an arrow on each tile you visit, the full path taken by the above example looks like this:

        >>v#    
        .#v.    
        #.v.    
        ..v.    
...#...v..v#    
>>>v...>#.>>    
..#v...#....    
...>>>>v..#.    
        ...#....
        .....#..
        .#......
        ......#.
To finish providing the password to this strange input device, you need to determine numbers for your final row, column, and facing as your final position appears from the perspective of the original map. Rows start from 1 at the top and count downward; columns start from 1 at the left and count rightward. (In the above example, row 1, column 1 refers to the empty space with no tile on it in the top-left corner.) Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^). The final password is the sum of 1000 times the row, 4 times the column, and the facing.

In the above example, the final row is 6, the final column is 8, and the final facing is 0. So, the final password is 1000 * 6 + 4 * 8 + 0: 6032.

Follow the path given in the monkeys' notes. What is the final password?

--- Part Two ---
As you reach the force field, you think you hear some Elves in the distance. Perhaps they've already arrived?

You approach the strange input device, but it isn't quite what the monkeys drew in their notes. Instead, you are met with a large cube; each of its six faces is a square of 50x50 tiles.

To be fair, the monkeys' map does have six 50x50 regions on it. If you were to carefully fold the map, you should be able to shape it into a cube!

In the example above, the six (smaller, 4x4) faces of the cube are:

        1111
        1111
        1111
        1111
222233334444
222233334444
222233334444
222233334444
        55556666
        55556666
        55556666
        55556666
You still start in the same position and with the same facing as before, but the wrapping rules are different. Now, if you would walk off the board, you instead proceed around the cube. From the perspective of the map, this can look a little strange. In the above example, if you are at A and move to the right, you would arrive at B facing down; if you are at C and move down, you would arrive at D facing up:

        ...#
        .#..
        #...
        ....
...#.......#
........#..A
..#....#....
.D........#.
        ...#..B.
        .....#..
        .#......
        ..C...#.
Walls still block your path, even if they are on a different face of the cube. If you are at E facing up, your movement is blocked by the wall marked by the arrow:

        ...#
        .#..
     -->#...
        ....
...#..E....#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.
Using the same method of drawing the last facing you had with an arrow on each tile you visit, the full path taken by the above example now looks like this:

        >>v#    
        .#v.    
        #.v.    
        ..v.    
...#..^...v#    
.>>>>>^.#.>>    
.^#....#....    
.^........#.    
        ...#..v.
        .....#v.
        .#v<<<<.
        ..v...#.
The final password is still calculated from your final position and facing from the perspective of the map. In this example, the final row is 5, the final column is 7, and the final facing is 3, so the final password is 1000 * 5 + 4 * 7 + 3 = 5031.

Fold the map into a cube, then follow the path given in the monkeys' notes. What is the final password?
*/
use std::collections::HashMap;

use crate::challenges::quadrant::get_quadrant;

use super::{
    coordinates::{new_coordinates, Coordinates, Move},
    direction::{oposite_direction, Direction},
    quadrant::Quadrant,
    state::{build_starting_state, MoveState, Rotate, State},
};

pub fn solve_first_challenge(
    coords: &HashMap<Coordinates, String>,
    instructions: &[String],
    starting_coords: &Coordinates,
) {
    solve_challenge(coords, instructions, starting_coords, false);
}

pub fn solve_second_challenge(
    coords: &HashMap<Coordinates, String>,
    instructions: &[String],
    starting_coords: &Coordinates,
) {
    solve_challenge(coords, instructions, starting_coords, true);
}

pub fn solve_challenge(
    coords: &HashMap<Coordinates, String>,
    instructions: &[String],
    starting_coords: &Coordinates,
    part2: bool,
) {
    let mut state: State = build_starting_state(*starting_coords);

    for (_, instruction) in instructions.iter().enumerate() {
        if is_numeric_string(instruction) {
            let num_of_moves = instruction.parse::<i64>().unwrap();
            for _ in 0..num_of_moves {
                let mut next_coords = state.peek();
                let mut new_direction: Direction = state.current_direction;
                if !coords.contains_key(&next_coords) {
                    if part2 {
                        (next_coords, new_direction) = get_wrap_around_coords_part_2(
                            &state.current_coords,
                            state.current_direction,
                        );
                    } else {
                        next_coords = get_wrap_around_coords_part_1(coords, &state);
                    }
                }

                if coords.get(&next_coords).unwrap().trim().starts_with('.') {
                    state.move_to_new_coords(next_coords);
                    if state.current_direction != new_direction {
                        state.rotate_to(new_direction);
                    }
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

fn get_wrap_around_coords_part_1(
    coords: &HashMap<Coordinates, String>,
    state: &State,
) -> Coordinates {
    let mut curr_coords = state
        .current_coords
        .peek(oposite_direction(state.current_direction));
    let mut next_coords: Coordinates = new_coordinates(curr_coords.x, curr_coords.y);
    while coords.contains_key(&curr_coords) {
        next_coords = new_coordinates(curr_coords.x, curr_coords.y);
        curr_coords = curr_coords.peek(oposite_direction(state.current_direction));
    }
    next_coords
}

fn get_wrap_around_coords_part_2(
    coords: &Coordinates,
    direction: Direction,
) -> (Coordinates, Direction) {
    let current_quadrant = get_quadrant(coords);
    if current_quadrant == Quadrant::A && direction == Direction::Left {
        (new_coordinates(151 - coords.x, 1), Direction::Right)
    } else if current_quadrant == Quadrant::A && direction == Direction::Up {
        (new_coordinates(100 + coords.y, 1), Direction::Right)
    } else if current_quadrant == Quadrant::B && direction == Direction::Up {
        (new_coordinates(200, coords.y - 100), Direction::Up)
    } else if current_quadrant == Quadrant::B && direction == Direction::Right {
        (new_coordinates(151 - coords.x, 100), Direction::Left)
    } else if current_quadrant == Quadrant::B && direction == Direction::Down {
        (new_coordinates(coords.y - 50, 100), Direction::Left)
    } else if current_quadrant == Quadrant::C && direction == Direction::Left {
        (new_coordinates(101, coords.x - 50), Direction::Down)
    } else if current_quadrant == Quadrant::C && direction == Direction::Right {
        (new_coordinates(50, coords.x + 50), Direction::Up)
    } else if current_quadrant == Quadrant::D && direction == Direction::Up {
        (new_coordinates(coords.y + 50, 51), Direction::Right)
    } else if current_quadrant == Quadrant::D && direction == Direction::Left {
        (new_coordinates(151 - coords.x, 51), Direction::Right)
    } else if current_quadrant == Quadrant::E && direction == Direction::Right {
        (new_coordinates(151 - coords.x, 150), Direction::Left)
    } else if current_quadrant == Quadrant::E && direction == Direction::Down {
        (new_coordinates(coords.y + 100, 50), Direction::Left)
    } else if current_quadrant == Quadrant::F && direction == Direction::Left {
        (new_coordinates(1, coords.x - 100), Direction::Down)
    } else if current_quadrant == Quadrant::F && direction == Direction::Right {
        (new_coordinates(150, coords.x - 100), Direction::Up)
    } else if current_quadrant == Quadrant::F && direction == Direction::Down {
        (new_coordinates(1, coords.y + 100), Direction::Down)
    } else {
        (new_coordinates(-1, -1), Direction::Left)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_around_quadrant_a_direction_up() {
        let direction = Direction::Up;
        let source_quadrant = Quadrant::A;
        let destination_quadrant = Quadrant::F;
        let received_direction = Direction::Right;

        let coords1 = new_coordinates(1, 51);
        let coords2 = new_coordinates(1, 100);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 151);
        assert_eq!(next_coords.y, 1);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 200);
        assert_eq!(next_coords.y, 1);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_a_direction_left() {
        let direction = Direction::Left;
        let source_quadrant = Quadrant::A;
        let destination_quadrant = Quadrant::D;
        let received_direction = Direction::Right;

        let coords1 = new_coordinates(1, 51);
        let coords2 = new_coordinates(50, 51);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 150);
        assert_eq!(next_coords.y, 1);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 101);
        assert_eq!(next_coords.y, 1);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_b_direction_up() {
        let direction = Direction::Up;
        let source_quadrant = Quadrant::B;
        let destination_quadrant = Quadrant::F;
        let received_direction = Direction::Up;

        let coords1 = new_coordinates(1, 101);
        let coords2 = new_coordinates(1, 150);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 200);
        assert_eq!(next_coords.y, 1);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 200);
        assert_eq!(next_coords.y, 50);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_b_direction_right() {
        let direction = Direction::Right;
        let source_quadrant = Quadrant::B;
        let destination_quadrant = Quadrant::E;
        let received_direction = Direction::Left;

        let coords1 = new_coordinates(1, 150);
        let coords2 = new_coordinates(50, 150);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 150);
        assert_eq!(next_coords.y, 100);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 101);
        assert_eq!(next_coords.y, 100);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_b_direction_down() {
        let direction = Direction::Down;
        let source_quadrant = Quadrant::B;
        let destination_quadrant = Quadrant::C;
        let received_direction = Direction::Left;

        let coords1 = new_coordinates(50, 101);
        let coords2 = new_coordinates(50, 150);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 51);
        assert_eq!(next_coords.y, 100);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 100);
        assert_eq!(next_coords.y, 100);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_c_direction_left() {
        let direction = Direction::Left;
        let source_quadrant = Quadrant::C;
        let destination_quadrant = Quadrant::D;
        let received_direction = Direction::Down;

        let coords1 = new_coordinates(51, 51);
        let coords2 = new_coordinates(100, 51);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 101);
        assert_eq!(next_coords.y, 1);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 101);
        assert_eq!(next_coords.y, 50);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_c_direction_right() {
        let direction = Direction::Right;
        let source_quadrant = Quadrant::C;
        let destination_quadrant = Quadrant::B;
        let received_direction = Direction::Up;

        let coords1 = new_coordinates(51, 100);
        let coords2 = new_coordinates(100, 100);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 50);
        assert_eq!(next_coords.y, 101);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 50);
        assert_eq!(next_coords.y, 150);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_d_direction_up() {
        let direction = Direction::Up;
        let source_quadrant = Quadrant::D;
        let destination_quadrant = Quadrant::C;
        let received_direction = Direction::Right;

        let coords1 = new_coordinates(101, 1);
        let coords2 = new_coordinates(101, 50);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 51);
        assert_eq!(next_coords.y, 51);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 100);
        assert_eq!(next_coords.y, 51);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_d_direction_left() {
        let direction = Direction::Left;
        let source_quadrant = Quadrant::D;
        let destination_quadrant = Quadrant::A;
        let received_direction = Direction::Right;

        let coords1 = new_coordinates(101, 1);
        let coords2 = new_coordinates(150, 1);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 50);
        assert_eq!(next_coords.y, 51);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 1);
        assert_eq!(next_coords.y, 51);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_e_direction_right() {
        let direction = Direction::Right;
        let source_quadrant = Quadrant::E;
        let destination_quadrant = Quadrant::B;
        let received_direction = Direction::Left;

        let coords1 = new_coordinates(101, 100);
        let coords2 = new_coordinates(150, 100);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 50);
        assert_eq!(next_coords.y, 150);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 1);
        assert_eq!(next_coords.y, 150);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_e_direction_down() {
        let direction = Direction::Down;
        let source_quadrant = Quadrant::E;
        let destination_quadrant = Quadrant::F;
        let received_direction = Direction::Left;

        let coords1 = new_coordinates(150, 51);
        let coords2 = new_coordinates(150, 100);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 151);
        assert_eq!(next_coords.y, 50);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 200);
        assert_eq!(next_coords.y, 50);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_f_direction_left() {
        let direction = Direction::Left;
        let source_quadrant = Quadrant::F;
        let destination_quadrant = Quadrant::A;
        let received_direction = Direction::Down;

        let coords1 = new_coordinates(151, 1);
        let coords2 = new_coordinates(200, 1);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 1);
        assert_eq!(next_coords.y, 51);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 1);
        assert_eq!(next_coords.y, 100);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_f_direction_down() {
        let direction = Direction::Down;
        let source_quadrant = Quadrant::F;
        let destination_quadrant = Quadrant::B;
        let received_direction = Direction::Down;

        let coords1 = new_coordinates(200, 1);
        let coords2 = new_coordinates(200, 50);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 1);
        assert_eq!(next_coords.y, 101);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 1);
        assert_eq!(next_coords.y, 150);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }

    #[test]
    fn wrap_around_quadrant_f_direction_right() {
        let direction = Direction::Right;
        let source_quadrant = Quadrant::F;
        let destination_quadrant = Quadrant::E;
        let received_direction = Direction::Up;

        let coords1 = new_coordinates(151, 50);
        let coords2 = new_coordinates(200, 50);
        assert_eq!(get_quadrant(&coords1), source_quadrant);
        assert_eq!(get_quadrant(&coords2), source_quadrant);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords1, direction);
        assert_eq!(next_coords.x, 150);
        assert_eq!(next_coords.y, 51);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);

        let (next_coords, new_direction) = get_wrap_around_coords_part_2(&coords2, direction);
        assert_eq!(next_coords.x, 150);
        assert_eq!(next_coords.y, 100);
        assert_eq!(get_quadrant(&next_coords), destination_quadrant);
        assert_eq!(new_direction, received_direction);
    }
}
