/* --- Day 8: Treetop Tree House ---
The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

30373
25512
65332
33549
35390
Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
The top-middle 5 is visible from the top and right.
The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
The left-middle 5 is visible, but only from the right.
The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
The right-middle 3 is visible from the right.
In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?

--- Part Two ---
Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.

To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances will be zero.)

The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.

In the example above, consider the middle 5 in the second row:

30373
25512
65332
33549
35390
Looking up, its view is not blocked; it can see 1 tree (of height 3).
Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
Looking right, its view is not blocked; it can see 2 trees.
Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).
A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).

However, you can do even better: consider the tree of height 5 in the middle of the fourth row:

30373
25512
65332
33549
35390
Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
Looking left, its view is not blocked; it can see 2 trees.
Looking down, its view is also not blocked; it can see 1 tree.
Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.

Consider each tree on your map. What is the highest scenic score possible for any tree?*/

use std::cmp::max;

use super::load_data::build_two_dimensional_tree_vector;
//use super::load_data::print_tree_vector;

pub fn solve_challenge(filename: &str) {
    let trees = build_two_dimensional_tree_vector(filename);
    //print_tree_vector(&trees);

    let mut max_visibility_score: u32 = 0;

    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            let tree_visibility_score: u32 =
                calculate_tree_visibility_score(&trees, Coordinates { x: i, y: j });
            max_visibility_score = max(max_visibility_score, tree_visibility_score);
        }
    }

    println!("Max visibility score: {}", max_visibility_score);
}

struct Coordinates {
    x: usize,
    y: usize,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn calculate_tree_visibility_score(trees: &Vec<Vec<u32>>, coordinates: Coordinates) -> u32 {
    calculate_visibility(trees, &coordinates, Direction::Up) *
    calculate_visibility(trees, &coordinates, Direction::Down) *
    calculate_visibility(trees, &coordinates, Direction::Left) *
    calculate_visibility(trees, &coordinates, Direction::Right)
}

fn calculate_visibility(
    trees: &Vec<Vec<u32>>,
    coordinates: &Coordinates,
    direction: Direction,
) -> u32 {
    let mut visibility_score: u32 = 0;
    let tree_height: u32 = trees[coordinates.x][coordinates.y];
    let mut traverse_coordinates: Coordinates = Coordinates {
        x: coordinates.x,
        y: coordinates.y,
    };

    loop {
        if is_loop_end(trees, &traverse_coordinates, &direction) {
            break;
        }

        traverse_coordinates = perform_move(&traverse_coordinates, &direction);
        visibility_score += 1;

        if tree_height <= trees[traverse_coordinates.x][traverse_coordinates.y] {
            break;
        }
    }
    visibility_score
}

fn is_loop_end(trees: &Vec<Vec<u32>>, coordinates: &Coordinates, direction: &Direction) -> bool {
    match direction {
        Direction::Up => coordinates.x == 0,
        Direction::Left => coordinates.y == 0,
        Direction::Right => coordinates.y == trees[0].len() - 1,
        Direction::Down => coordinates.x == trees.len() - 1,
    }
}

fn perform_move(current_coordinates: &Coordinates, direction: &Direction) -> Coordinates {
    let new_row = match direction {
        Direction::Down => current_coordinates.x + 1,
        Direction::Up => current_coordinates.x - 1,
        Direction::Right => current_coordinates.x,
        Direction::Left => current_coordinates.x,
    };

    let new_column: usize = match direction {
        Direction::Down => current_coordinates.y,
        Direction::Up => current_coordinates.y,
        Direction::Right => current_coordinates.y + 1,
        Direction::Left => current_coordinates.y - 1,
    };

    Coordinates {
        x: new_row,
        y: new_column,
    }
}


