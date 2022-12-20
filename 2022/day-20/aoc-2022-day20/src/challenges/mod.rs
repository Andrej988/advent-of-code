mod read_lines;
mod parse_lines;
mod challenge;

pub fn parse_numbers(filename: &str) -> Vec<i64> {
    parse_lines::parse_lines(filename)
}

pub fn solve_first_challenge(parsed_numbers: &Vec<i64>) {
    challenge::solve_first_challenge(parsed_numbers);
}

pub fn solve_second_challenge(parsed_numbers: &Vec<i64>) {
    challenge::solve_second_challenge(parsed_numbers);
}