use super::coordinates::{new_coordinates, Coordinates};
use std::collections::{HashMap, HashSet};

pub trait Dijkstra {
    fn new(nodes: HashSet<Coordinates>, starting_point: Coordinates) -> Self;
    fn update_value(
        &mut self,
        vertex: Coordinates,
        shortes_distance: u32,
        previous_vertex: Coordinates,
    );
    fn get_next_unvisited_node(&self, unvisited: &HashSet<Coordinates>) -> Coordinates;
    fn get_distance_from_start(&self, vertex: Coordinates) -> u32;
    fn print(&self);
}

pub struct HashMapDijkstra {
    vertexes: HashMap<Coordinates, HaspMapDijkstraValue>,
}

pub struct HaspMapDijkstraValue {
    shortest_distance_from_start: u32,
    previous_vertex: Coordinates,
}

impl Dijkstra for HashMapDijkstra {
    fn new(nodes: HashSet<Coordinates>, starting_point: Coordinates) -> HashMapDijkstra {
        let mut vertextes: HashMap<Coordinates, HaspMapDijkstraValue> = HashMap::new();
        for node in nodes {
            let value = HaspMapDijkstraValue {
                shortest_distance_from_start: u32::MAX,
                previous_vertex: starting_point,
            };
            vertextes.insert(node, value);
        }
        vertextes.insert(
            starting_point,
            HaspMapDijkstraValue {
                shortest_distance_from_start: 0,
                previous_vertex: starting_point,
            },
        );
        HashMapDijkstra {
            vertexes: vertextes,
        }
    }

    fn get_next_unvisited_node(&self, unvisited: &HashSet<Coordinates>) -> Coordinates {
        if unvisited.is_empty() {
            panic!("All nodes have been visited!!!");
        } else {
            let mut coord: Coordinates = new_coordinates(usize::MAX, usize::MAX);
            let mut shortest_path: u32 = u32::MAX;
            for item in unvisited.iter() {
                let item_current_shortest_path = self
                    .vertexes
                    .get(item)
                    .unwrap()
                    .shortest_distance_from_start;
                if item_current_shortest_path < shortest_path {
                    coord = *item;
                    shortest_path = item_current_shortest_path;
                }
            }
            coord
        }
    }

    fn update_value(
        &mut self,
        vertex: Coordinates,
        shortest_distance: u32,
        previous_vertex: Coordinates,
    ) {
        self.vertexes.insert(
            vertex,
            HaspMapDijkstraValue {
                shortest_distance_from_start: shortest_distance,
                previous_vertex,
            },
        );
    }

    fn get_distance_from_start(&self, vertex: Coordinates) -> u32 {
        let value = self.vertexes.get(&vertex).unwrap();
        value.shortest_distance_from_start
    }

    fn print(&self) {
        println!("------------------------------");
        println!("Dijkstras result: ");
        for (vertex, value) in self.vertexes.iter() {
            println!(
                "Vertex: {}; Shortest path from starting point: {}; Previous vertex: {}",
                vertex, value.shortest_distance_from_start, value.previous_vertex
            );
        }
        println!("------------------------------");
    }
}
