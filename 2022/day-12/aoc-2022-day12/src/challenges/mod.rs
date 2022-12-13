mod bfs_operation;
mod challenge_bfs;
mod challenge_dijkstra;
mod coordinates;
mod dijkstra;
mod direction;
mod elevations;
mod load_data;
mod read_lines;

use crate::challenges::challenge_bfs::solve_challenge1 as challenge_bfs;
use crate::challenges::challenge_bfs::solve_challenge2 as challenge_bfs2;
use crate::challenges::challenge_dijkstra::solve_challenge as challenge_dijkstra;
use crate::challenges::load_data::*;

use self::coordinates::Coordinates;

pub fn load_data_from_file(filename: &str) -> (Vec<Vec<u8>>, Coordinates, Coordinates) {
    build_two_dimensional_vector(filename)
}

#[allow(dead_code)]
pub fn print_data(elevation: &[Vec<u8>]) {
    print(elevation);
}

pub fn solve_first_challenge_bfs(
    elevation: &Vec<Vec<u8>>,
    start_point: &Coordinates,
    end_point: &Coordinates,
) {
    challenge_bfs(elevation, start_point, end_point);
}

pub fn solve_first_challenge_dijkstra(
    elevation: &Vec<Vec<u8>>,
    start_point: &Coordinates,
    end_point: &Coordinates,
) {
    challenge_dijkstra(elevation, start_point, end_point);
}

pub fn solve_second_challenge_bfs(
    elevation: &Vec<Vec<u8>>,
    start_point: &Coordinates,
    end_point: &Coordinates,
) {
    challenge_bfs2(elevation, start_point, end_point);
}
