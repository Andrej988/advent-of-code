use crate::challenges::{coordinates::new_coordinates, obstacles::ObstacleMap};

use super::{
    coordinates::Coordinates,
    obstacles::{Obstacle, Obstacles},
    read_lines::read_lines,
};

pub fn parse_lines(filename: &str) -> (Obstacles, Coordinates, Coordinates) {
    let mut obstacles: Obstacles = Obstacles::default();
    let mut starting_point: Coordinates = Coordinates::default();
    let mut target_point: Coordinates = Coordinates::default();

    if let Ok(lines) = read_lines(filename) {
        for (x, line) in lines.flatten().enumerate() {
            if !line.is_empty() {
                for (y, ch) in line.chars().enumerate() {
                    let coords = new_coordinates(x, y);
                    if ch == '.' && x == 0 {
                        starting_point = coords;
                    } else if ch == '.' {
                        target_point = coords;
                    } else {
                        match ch {
                            '#' => obstacles.add_to_map(coords, Obstacle::Wall),
                            '>' => obstacles.add_to_map(coords, Obstacle::BLizzardRight),
                            '<' => obstacles.add_to_map(coords, Obstacle::BlizzardLeft),
                            '^' => obstacles.add_to_map(coords, Obstacle::BlizzardUp),
                            'v' => obstacles.add_to_map(coords, Obstacle::BlizzardDown),
                            _ => println!("Wrong character!!!"),
                        }
                    }
                }
            }
        }
    }
    obstacles.calculate_next_positions();
    (obstacles, starting_point, target_point)
}
