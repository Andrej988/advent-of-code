//use crate::challenges::print_map;

mod challenges;

fn main() {
    const FILENAME: &str = "input.txt";

    println!("First challenge: ");
    let mut map = challenges::build_map_from_file(FILENAME);
    //print_map(&map);
    challenges::solve_first_challenge(&mut map);

    println!("----------------------");

    println!("Second challenge: ");
    map = challenges::build_map_from_file(FILENAME);
    //print_map(&map);
    challenges::solve_second_challenge(&mut map);
}
