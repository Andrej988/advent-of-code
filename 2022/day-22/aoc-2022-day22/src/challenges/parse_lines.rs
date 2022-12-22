use std::collections::HashMap;

use crate::challenges::coordinates::new_coordinates;

use super::{coordinates::Coordinates, read_lines::read_lines};

pub fn parse_lines(filename: &str) -> (HashMap<Coordinates, String>, Vec<String>, Coordinates) {
    let mut coords_map: HashMap<Coordinates, String> = HashMap::new();
    let mut instructions: Vec<String> = Vec::new();
    let mut starting_coords: Coordinates = new_coordinates(-1, -1);

    if let Ok(lines) = read_lines(filename) {
        for (x, line) in lines.flatten().enumerate() {
            if !line.is_empty() {
                if line.chars().next().unwrap().is_numeric() {
                    let mut digits: String = String::new();
                    for (i, ch) in line.chars().enumerate() {
                        if ch == ' ' {
                            continue;
                        } else if ch.is_numeric() {
                            digits.push(ch);
                            if i == line.len() - 1 || !line.chars().nth(i + 1).unwrap().is_numeric()
                            {
                                instructions.push(digits);
                                digits = String::new();
                            }
                        } else {
                            instructions.push(ch.to_string());
                        }
                    }
                } else {
                    for (y, ch) in line.chars().enumerate() {
                        if ch != ' ' {
                            if x == 0 && starting_coords.x < 0 {
                                starting_coords = new_coordinates((x + 1) as i64, (y + 1) as i64);
                            }
                            coords_map.insert(
                                new_coordinates((x + 1) as i64, (y + 1) as i64),
                                ch.to_string(),
                            );
                        }
                    }
                }
            }
        }
    }
    (coords_map, instructions, starting_coords)
}
