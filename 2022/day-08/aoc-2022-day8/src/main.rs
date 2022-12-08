mod challenges;

fn main() {
    let filename: &str = "input.txt";

    println!("First challenge: ");
    challenges::first_challenge(filename);

    println!("-----------------");
    println!("Second challenge: ");
    challenges::second_challenge(filename);
}
