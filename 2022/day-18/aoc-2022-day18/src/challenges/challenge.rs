/* --- Day 18: Boiling Boulders ---
You and the elephants finally reach fresh air. You've emerged near the base of a large volcano that seems to be actively erupting! Fortunately, the lava seems to be flowing away from you and toward the ocean.

Bits of lava are still being ejected toward you, so you're sheltering in the cavern exit a little longer. Outside the cave, you can see the lava landing in a pond and hear it loudly hissing as it solidifies.

Depending on the specific compounds in the lava and speed at which it cools, it might be forming obsidian! The cooling rate should be based on the surface area of the lava droplets, so you take a quick scan of a droplet as it flies past you (your puzzle input).

Because of how quickly the lava is moving, the scan isn't very good; its resolution is quite low and, as a result, it approximates the shape of the lava droplet with 1x1x1 cubes on a 3D grid, each given as its x,y,z position.

To approximate the surface area, count the number of sides of each cube that are not immediately connected to another cube. So, if your scan were only two adjacent cubes like 1,1,1 and 2,1,1, each cube would have a single side covered and five sides exposed, a total surface area of 10 sides.

Here's a larger example:

2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
In the above example, after counting up all the sides that aren't connected to another cube, the total surface area is 64.

What is the surface area of your scanned lava droplet?

--- Part Two ---
Something seems off about your calculation. The cooling rate depends on exterior surface area, but your calculation also included the surface area of air pockets trapped in the lava droplet.

Instead, consider only cube sides that could be reached by the water and steam as the lava droplet tumbles into the pond. The steam will expand to reach as much as possible, completely displacing any air on the outside of the lava droplet but never expanding diagonally.

In the larger example above, exactly one cube of air is trapped within the lava droplet (at 2,2,5), so the exterior surface area of the lava droplet is 58.

What is the exterior surface area of your scanned lava droplet?
*/
use std::collections::{HashMap, HashSet, VecDeque};

use crate::challenges::coordinates::get_possible_neighbours;

use super::cube::{get_max_sides, get_min_sides, new_cube, Cube};
use super::{
    coordinates::{new_coordinates, Coordinates},
    read_lines::read_lines,
};

pub fn solve_first_challenge(cubes: &HashMap<Coordinates, Cube>) {
    let total: u32 = cubes
        .iter()
        .map(|(_, v)| 6 - v.neighbours.len())
        .map(|v| u32::try_from(v).unwrap())
        .sum();
    println!("Total: {}", total);
}

pub fn solve_second_challenge_waterfill(cubes: &HashMap<Coordinates, Cube>) {
    let (min_x, min_y, min_z) = get_min_sides(cubes);
    let (max_x, max_y, max_z) = get_max_sides(cubes);

    let mut queue: VecDeque<Coordinates> = VecDeque::new();
    let mut filled_with_water: HashSet<Coordinates> = HashSet::new();
    let mut processed: HashSet<Coordinates> = HashSet::new();
    let starting_point = new_coordinates(min_x - 1, min_y - 1, min_z - 1);
    queue.push_back(starting_point);

    while !queue.is_empty() {
        let current_coords = queue.pop_front().unwrap();
        filled_with_water.insert(current_coords);
        processed.insert(current_coords);

        for n in get_possible_neighbours(current_coords) {
            // Already in queue or processed
            if processed.contains(&n) {
                //println!("Already in processing");
                continue;
            }

            //Out of bounds x, y, z
            if n.x < min_x - 1
                || n.x > max_x + 1
                || n.y < min_y - 1
                || n.y > max_y + 1
                || n.z < min_z - 1
                || n.z > max_z + 1
            {
                continue;
            }

            // Is cube - filling with water blocked
            if cubes.contains_key(&n) {
                continue;
            }

            queue.push_back(n);
            processed.insert(n);
        }
    }

    let total = cubes
        .iter()
        .flat_map(|(c, _)| get_possible_neighbours(*c))
        .filter(|c| filled_with_water.contains(c))
        .count();

    println!("Total: {}", total);
}

pub fn build_cube_map_from_lines(filename: &str) -> HashMap<Coordinates, Cube> {
    let mut cubes: HashMap<Coordinates, Cube> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for (_, line) in lines.flatten().enumerate() {
            if !line.is_empty() {
                let line_coords: Vec<&str> = line.split(',').collect();
                let x: i32 = line_coords[0].parse().unwrap();
                let y: i32 = line_coords[1].parse().unwrap();
                let z: i32 = line_coords[2].parse().unwrap();
                let coords = new_coordinates(x, y, z);
                cubes.insert(coords, new_cube(coords));
            }
        }
    }

    fill_neighbours(&mut cubes);

    //println!("Cubes: {:?}", cubes);
    cubes
}

fn fill_neighbours(cubes: &mut HashMap<Coordinates, Cube>) {
    let mut set: HashSet<Coordinates> = HashSet::new();
    cubes.iter().for_each(|e| {
        set.insert(*e.0);
    });

    cubes.iter_mut().for_each(|entry| {
        for p in get_possible_neighbours(*entry.0) {
            if set.contains(&p) {
                entry.1.neighbours.insert(p);
            }
        }
    });
}
