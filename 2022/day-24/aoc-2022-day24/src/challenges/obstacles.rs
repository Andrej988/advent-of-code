use std::collections::{HashMap, HashSet};

use super::{
    coordinates::{Coordinates, Move},
    direction::{oposite_direction, Direction},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Obstacle {
    Wall,
    BlizzardLeft,
    BLizzardRight,
    BlizzardUp,
    BlizzardDown,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Obstacles {
    pub blizzards: HashMap<Coordinates, Vec<Obstacle>>,
    pub next_blizzards: HashMap<Coordinates, Vec<Obstacle>>,
    pub walls: HashSet<Coordinates>,
}

pub trait ObstacleMap {
    fn add_to_map(&mut self, coords: Coordinates, obstacle: Obstacle);
    fn calculate_next_positions(&mut self);
    fn is_wall(&self, coords: &Coordinates) -> bool;
    fn is_blocked(&self, coords: &Coordinates) -> bool;
    fn is_blocked_in_next_iteration(&self, coords: &Coordinates) -> bool;
    fn move_blizzards(&mut self);
}

impl ObstacleMap for Obstacles {
    fn add_to_map(&mut self, coords: Coordinates, obstacle: Obstacle) {
        if obstacle == Obstacle::Wall {
            self.walls.insert(coords);
        } else {
            add_to_blizzard_map(&mut self.blizzards, coords, obstacle);
        }
    }

    fn calculate_next_positions(&mut self) {
        let mut new_blizzards: HashMap<Coordinates, Vec<Obstacle>> = HashMap::new();

        for (coords, obstacles) in self.blizzards.iter_mut() {
            for blizzard in obstacles.iter_mut() {
                let mut new_coords = coords.peek(get_blizzard_direction(*blizzard));
                if self.walls.contains(&new_coords) {
                    let oposite_direction = oposite_direction(get_blizzard_direction(*blizzard));
                    while !self.walls.contains(&new_coords.peek(oposite_direction)) {
                        new_coords = new_coords.peek(oposite_direction);
                    }
                }
                add_to_blizzard_map(&mut new_blizzards, new_coords, *blizzard);
            }
        }
        self.next_blizzards = new_blizzards;
    }
    fn is_wall(&self, coords: &Coordinates) -> bool {
        self.walls.contains(coords)
    }

    fn is_blocked(&self, coords: &Coordinates) -> bool {
        self.walls.contains(coords) || self.blizzards.contains_key(coords)
    }

    fn is_blocked_in_next_iteration(&self, coords: &Coordinates) -> bool {
        self.walls.contains(coords) || self.next_blizzards.contains_key(coords)
    }

    fn move_blizzards(&mut self) {
        self.blizzards = self.next_blizzards.clone();
        self.calculate_next_positions();
        /*let mut new_blizzards: HashMap<Coordinates, Vec<Obstacle>> = HashMap::new();

        for (coords, obstacles) in self.blizzards.iter_mut() {
            for blizzard in obstacles.iter_mut() {
                let mut new_coords = coords.peek(get_blizzard_direction(*blizzard));
                if self.walls.contains(&new_coords) {
                    let oposite_direction = oposite_direction(get_blizzard_direction(*blizzard));
                    while !self.walls.contains(&new_coords.peek(oposite_direction)) {
                        new_coords = new_coords.peek(oposite_direction);
                    }
                }
                add_to_blizzard_map(&mut new_blizzards, new_coords, *blizzard);
            }
        }
        self.blizzards = new_blizzards;*/
    }
}

fn get_blizzard_direction(obstacle: Obstacle) -> Direction {
    match obstacle {
        Obstacle::BLizzardRight => Direction::Right,
        Obstacle::BlizzardLeft => Direction::Left,
        Obstacle::BlizzardDown => Direction::Down,
        Obstacle::BlizzardUp => Direction::Up,
        Obstacle::Wall => todo!(),
    }
}

fn add_to_blizzard_map(
    blizzards: &mut HashMap<Coordinates, Vec<Obstacle>>,
    coords: Coordinates,
    blizzard: Obstacle,
) {
    if blizzards.contains_key(&coords) {
        let obstacles = blizzards.get(&coords).unwrap();
        let mut new_obstacles = obstacles.to_vec();
        new_obstacles.push(blizzard);
        blizzards.insert(coords, new_obstacles);
    } else {
        let obstacles: Vec<Obstacle> = vec![blizzard];
        blizzards.insert(coords, obstacles);
    }
}
