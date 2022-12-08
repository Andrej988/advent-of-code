use super::read_lines::read_lines;

pub fn build_two_dimensional_tree_vector(filename: &str) -> Vec<Vec<u32>> {
    let mut trees: Vec<Vec<u32>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            if !line.is_empty() {
                process_line(&line, &mut trees);
            }
        }
    }
    trees
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

pub fn print_tree_vector(trees: &[Vec<u32>]) {
    for tree_row in trees.iter() {
        for tree in tree_row {
            print!(" {} ", tree);
        }
        println!();
    }
}
