use super::{
    coordinates::{new_coordinates, Coordinates},
    read_lines::read_lines,
};

pub fn build_two_dimensional_vector(filename: &str) -> (Vec<Vec<u8>>, Coordinates, Coordinates) {
    let mut elevation: Vec<Vec<u8>> = Vec::new();
    let mut start_point: Coordinates = new_coordinates(1000, 1000);
    let mut end_point: Coordinates = new_coordinates(1000, 1000);

    if let Ok(lines) = read_lines(filename) {
        for (x, line) in lines.flatten().enumerate() {
            if !line.is_empty() {
                let new_row: Vec<char> = line.chars().collect();
                let mut values: Vec<u8> = Vec::new();

                for (y, col) in new_row.iter().enumerate() {
                    if *col == 'S' {
                        start_point = new_coordinates(x, y);
                    } else if *col == 'E' {
                        end_point = new_coordinates(x, y);
                    }
                    values.push(char_to_int_value(*col))
                }
                elevation.push(values);
            }
        }
    }
    (elevation, start_point, end_point)
}

#[allow(dead_code)]
pub fn print(elevation: &[Vec<u8>]) {
    for row in elevation.iter() {
        for col in row {
            print!(" {}, ", col);
        }
        println!();
    }
}

fn char_to_int_value(c: char) -> u8 {
    let val: u8;
    if c == 'S' {
        val = b'a';
    } else if c == 'E' {
        val = b'z';
    } else {
        val = c as u8;
    }
    val - 96
}
