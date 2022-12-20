use super::{read_lines::read_lines};

pub fn parse_lines(filename: &str) -> Vec<i64> {
    let mut nums: Vec<i64> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for (_, line) in lines.flatten().enumerate() {
            if !line.is_empty() {
                let x: i64 = line.parse().unwrap();
                nums.push(x);
            }
        }
    }
    nums
}