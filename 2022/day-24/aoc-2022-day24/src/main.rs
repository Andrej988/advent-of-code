mod challenges;

fn main() {
    const FILENAME: &str = "input.txt";

    println!("First challenge:");
    challenges::solve_first_challenge(FILENAME);

    println!("-----------------");

    println!("Second challenge:");
    challenges::solve_second_challenge(FILENAME);
}
