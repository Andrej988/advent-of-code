use std::collections::HashSet;

use super::coordinates::*;
use super::dijkstra::*;
use super::direction::*;
use super::elevations::elevation_to_set;

pub fn solve_challenge(
    elevation: &Vec<Vec<u8>>,
    start_point: &Coordinates,
    end_point: &Coordinates,
) {
    let vertexes: HashSet<Coordinates> = elevation_to_set(elevation);
    let mut visited: HashSet<Coordinates> = HashSet::new();
    let mut unvisited: HashSet<Coordinates> = elevation_to_set(elevation);

    let mut dijkstras: HashMapDijkstra = Dijkstra::new(vertexes, *start_point);

    while !unvisited.is_empty() {
        let current_coordinates: Coordinates = dijkstras.get_next_unvisited_node(&unvisited);
        if current_coordinates.x == usize::MAX || current_coordinates.y == usize::MAX {
            break;
        }

        unvisited.remove(&current_coordinates);
        visited.insert(current_coordinates);

        process_vertex(elevation, current_coordinates, &mut dijkstras);
    }

    println!(
        "Shortest path: {}",
        dijkstras.get_distance_from_start(*end_point)
    );
}

fn process_vertex(
    elevation: &Vec<Vec<u8>>,
    current_coordinates: Coordinates,
    dijkstras: &mut HashMapDijkstra,
) {
    for direction in Direction::iterator() {
        if is_possible_move_direction(elevation, current_coordinates, direction) {
            let new_coordinates = perform_move(&current_coordinates, direction);
            if is_allowed_to_move(elevation, current_coordinates, new_coordinates) {
                let path_to_start: u32 = dijkstras.get_distance_from_start(current_coordinates) + 1;

                let shortest_path_so_far: u32 = dijkstras.get_distance_from_start(new_coordinates);
                if path_to_start < shortest_path_so_far {
                    dijkstras.update_value(new_coordinates, path_to_start, current_coordinates)
                }
            }
        }
    }
}
