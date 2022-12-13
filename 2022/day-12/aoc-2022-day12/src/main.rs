mod challenges;

fn main() {
    const FILENAME: &str = "input.txt";
    let (elevation_map, start_point, end_point) = challenges::load_data_from_file(FILENAME);
    //challenges::print_data(&elevation_map);
    println!("Start point: {}", start_point);
    println!("End point: {}", end_point);
    println!("-----------------------------");
    println!();

    println!("First challenge Dijkstra: ");
    challenges::solve_first_challenge_dijkstra(&elevation_map, &start_point, &end_point);
    println!("-----------------------------");
    println!();

    println!("First challenge BFS: ");
    challenges::solve_first_challenge_bfs(&elevation_map, &start_point, &end_point);
    println!("-----------------------------");
    println!();

    println!("Second challenge BFS: ");
    challenges::solve_second_challenge_bfs(&elevation_map, &start_point, &end_point);
    println!("-----------------------------");
    println!();
}
