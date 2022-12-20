mod challenges;

fn main() {
    const FILENAME: &str = "input.txt";
    let original_arr = challenges::parse_numbers(FILENAME);

    println!("First challenge:");
    challenges::solve_first_challenge(&original_arr);

    println!("-----------------");

    println!("Second challenge:");
    challenges::solve_second_challenge(&original_arr);
}