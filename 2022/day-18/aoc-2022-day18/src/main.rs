mod challenges;

fn main() {
    const FILENAME: &str = "input.txt";

    let cubes = challenges::build_cube_map(FILENAME);

    println!("First challenge:");
    challenges::solve_first_challenge(&cubes);

    println!("-----------------");

    println!("Second challenge:");
    challenges::solve_second_challenge(&cubes);
}
