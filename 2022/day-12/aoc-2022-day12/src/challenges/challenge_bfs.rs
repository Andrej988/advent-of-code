use std::collections::HashSet;
use std::collections::VecDeque;

use super::bfs_operation::new_bfs_operation;
use super::bfs_operation::BFSOperation;
use super::coordinates::*;
use super::direction::*;

pub fn solve_challenge1(
    elevation: &Vec<Vec<u8>>,
    start_point: &Coordinates,
    end_point: &Coordinates,
) {
    let shortest_path: i32 = solve_using_bfs(elevation, end_point, start_point, false);
    println!("Shortest path: {}", shortest_path);
}

pub fn solve_challenge2(
    elevation: &Vec<Vec<u8>>,
    start_point: &Coordinates,
    end_point: &Coordinates,
) {
    let shortest_path: i32 = solve_using_bfs(elevation, end_point, start_point, true);
    println!("Shortest path: {}", shortest_path);
}

fn solve_using_bfs(
    elevation: &Vec<Vec<u8>>,
    start_node: &Coordinates,
    end_node: &Coordinates,
    challenge2: bool,
) -> i32 {
    let mut queue: VecDeque<BFSOperation> = VecDeque::new();
    queue.push_front(new_bfs_operation(*start_node, 0));

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(get_coordinates_key(start_node));

    let mut previous: Vec<Coordinates> = Vec::new();

    while !queue.is_empty() {
        let operation = queue.pop_front().unwrap();
        let node = operation.node;
        let neighbours: Vec<Coordinates> = get_neighbours(elevation, node);

        if (!challenge2 && node.x == end_node.x && node.y == end_node.y)
            || (challenge2 && elevation[node.x][node.y] == 1)
        {
            return i32::try_from(operation.level).unwrap();
        }

        for next in neighbours.iter() {
            let key: String = get_coordinates_key(next);
            if !visited.contains(&key) {
                queue.push_back(new_bfs_operation(*next, operation.level + 1));
                visited.insert(key);
                previous.push(node);
            }
        }
    }
    -1
}

fn get_neighbours(elevation: &Vec<Vec<u8>>, node: Coordinates) -> Vec<Coordinates> {
    let mut next_steps: Vec<Coordinates> = Vec::new();

    for direction in Direction::iterator() {
        if is_possible_move_direction(elevation, node, direction) {
            let new_coordinates = perform_move(&node, direction);
            if is_allowed_to_move_reverse(elevation, node, new_coordinates) {
                next_steps.push(new_coordinates);
            }
        }
    }
    next_steps
}
