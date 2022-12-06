/* See challenge.rsfor instructions */

mod challenges;

fn main() {
    let filename: &str = "input.txt";

    println!("First challenge:");
    challenges::first_challenge(filename);

    println!("Second challenge:");
    challenges::second_challenge(filename);
}
