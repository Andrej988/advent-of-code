use super::read_lines::read_lines;

pub fn build_commands_vector(filename: &str) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            if !line.is_empty() {
                commands.push(line);
            }
        }
    }
    commands
}

#[allow(dead_code)]
pub fn print_commands(commands: &[String]) {
    for cmd in commands.iter() {
        println!("{}", cmd);
    }
}
