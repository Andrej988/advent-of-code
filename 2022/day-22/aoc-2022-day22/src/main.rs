mod challenges;

fn main() {
    const FILENAME: &str = "input.txt";

    let (coords, instructions, starting_coords) = challenges::parse_input(FILENAME);
    //let original_arr = challenges::parse_numbers(FILENAME);

    println!("First challenge:");
    challenges::solve_first_challenge(&coords, &instructions, &starting_coords);

    println!("-----------------");

    /*println!("Second challenge:");
    challenges::solve_second_challenge(&original_arr);*/
}
