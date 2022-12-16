mod challenges;

fn main() {
    const FILENAME: &str = "input.txt";

    let valves = challenges::build_valves_map(FILENAME);

    println!("First challenge");
    challenges::solve_first_challenge(&valves);
}
