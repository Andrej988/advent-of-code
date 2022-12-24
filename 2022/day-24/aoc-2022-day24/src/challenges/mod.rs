mod challenge;
mod coordinates;
mod direction;
mod obstacles;
mod parse_lines;
mod read_lines;

pub fn solve_first_challenge(filename: &str) {
    let (obstacles, starting_point, target_point) = parse_lines::parse_lines(filename);
    challenge::solve_first_challenge(obstacles, starting_point, target_point);
}

pub fn solve_second_challenge(filename: &str) {
    let (obstacles, starting_point, target_point) = parse_lines::parse_lines(filename);
    challenge::solve_second_challenge(obstacles, starting_point, target_point);
}
