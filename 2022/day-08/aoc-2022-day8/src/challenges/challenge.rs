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

Consider your map; how many trees are visible from outside the grid? */

use super::read_lines::read_lines;

pub fn solve_challenge(filename: &str) {
    let trees = build_two_dimensional_tree_vector(filename);
    print_tree_vector(&trees);

    let mut total_visible_trees: u32 = 0;

    for i in 0 .. trees.len() {
        for j in 0 .. trees[i].len() {
            if determine_if_tree_is_visible(&trees, i, j) {
                total_visible_trees += 1;
            }
        }
    }

    println!("Total visible trees: {}", total_visible_trees);
}

fn print_tree_vector(trees: &Vec<Vec<u32>>) {
    for tree_row in trees.iter() {
        for tree in tree_row {
            print!(" {} ", tree);
        }
        print!("\n");
    }
}

fn build_two_dimensional_tree_vector(filename: &str) -> Vec<Vec<u32>> {
    let mut trees: Vec<Vec<u32>> = Vec::new();
    
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            if !line.is_empty() {
                process_line(&line, &mut trees);
            }
        }
    }
    return trees;
}

fn process_line(line_text: &str, trees: &mut Vec<Vec<u32>>) {
    const RADIX: u32 = 10;
    let mut new_tree_row: Vec<u32> = Vec::new();

    let digits: Vec<char> = line_text.chars().collect();
    for char in digits {
        let tree_height: u32 = char.to_digit(RADIX).unwrap();
        new_tree_row.push(tree_height);
    }

    trees.push(new_tree_row);
}

fn determine_if_tree_is_visible(trees: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    if row == 0 || row == trees.len() -1 || column == 0 || column == trees[row].len() -1 {
        
        return true;
    } else {
        return is_visible_from_left(trees, row, column) || is_visible_from_right(trees, row, column) || 
            is_visible_from_top(trees, row, column) || is_visible_from_bottom(trees, row, column);
    }
}

fn is_visible_from_left(trees: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    let mut idx: usize = column;

    while idx > 0 {
        idx -= 1;
        if trees[row][idx] >= trees[row][column] {
            return false;
        }
    }
    return true;
}

fn is_visible_from_right(trees: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    let mut idx: usize = column + 1;

    while idx < trees[row].len() {
        if trees[row][idx] >= trees[row][column] {
            return false;
        }

        idx += 1;
    }
    return true;
}

fn is_visible_from_top(trees: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    let mut idx: usize = row;

    while idx > 0 {
        idx -= 1;
        if trees[idx][column] >= trees[row][column] {
            return false;
        }
    }
    return true;
}

fn is_visible_from_bottom(trees: &Vec<Vec<u32>>, row: usize, column: usize) -> bool {
    let mut idx: usize = row + 1;

    while idx < trees[column].len() {
        if trees[idx][column] >= trees[row][column] {
            return false;
        }

        idx += 1;
    }
    return true;
}