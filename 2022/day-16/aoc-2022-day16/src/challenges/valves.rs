use std::collections::HashMap;

use super::read_lines::read_lines;

#[derive(Debug)]
pub struct Valve {
    pub id: u8,
    pub name: String,
    pub flow_rate: u32,
    pub tunnels: Vec<String>,
}

pub fn new_valve(id: u8, name: String, flow_rate: u32, tunnels: Vec<String>) -> Valve {
    Valve {
        id,
        name,
        flow_rate,
        tunnels,
    }
}

pub fn build_valves_map(filename: &str) -> HashMap<String, Valve> {
    let mut valves: HashMap<String, Valve> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for (i, line) in lines.flatten().enumerate() {
            if !line.is_empty() {
                let name = &line[6..8];
                let flow_rate_from = line.find("rate=").unwrap() + 5;
                let flow_rate_to = line.find(';').unwrap();
                let flow_rate: u32 = line[flow_rate_from..flow_rate_to].parse().unwrap();
                let tunnels_from_idx = line.find("tunnel").unwrap() + 22;
                let tunnels_from: Vec<&str> = line[tunnels_from_idx..].split(',').collect();

                let mut tunnels: Vec<String> = Vec::new();
                for x in tunnels_from.iter() {
                    tunnels.push(x.replace(' ', ""));
                }
                valves.insert(
                    String::from(name),
                    new_valve(
                        u8::try_from(i).unwrap(),
                        String::from(name),
                        flow_rate,
                        tunnels,
                    ),
                );
            }
        }
    }
    valves
}
