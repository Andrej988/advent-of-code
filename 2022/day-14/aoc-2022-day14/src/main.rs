//use crate::challenges::print_map;

mod challenges;

fn main() {
    const FILENAME: &str = "input.txt";
    let mut map = challenges::build_map_from_file(FILENAME);
    //print_map(&map);

    println!("First challenge: ");
    challenges::solve_first_challenge(&mut map);
}
