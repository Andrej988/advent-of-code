use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use super::valves::Valve;

pub fn solve_first_challenge(valves: &HashMap<String, Valve>) {
    let starting_valve = &valves["AA"];
    let result = max_flow(valves, starting_valve, &HashSet::new(), 30);
    println!("Result: {}", result);
}

fn max_flow(
    valves: &HashMap<String, Valve>,
    current_valve: &Valve,
    opened: &HashSet<String>,
    min_left: u32,
) -> u32 {
    if min_left == 0 {
        return 0;
    }

    //let valve = current_valve.borrow().clone();
    let valve = &current_valve;

    let mut best: u32 = 0;
    if !opened.contains(&valve.name) {
        let val = (min_left - 1) * valve.flow_rate;
        let mut cur_opened: HashSet<String> = HashSet::new();
        for o in opened.iter() {
            cur_opened.insert(o.to_string());
        }
        cur_opened.insert(String::from(&valve.name));
        for tunnel in valve.tunnels.iter() {
            let next_valve = &valves[tunnel];
            if val != 0 {
                best = cmp::max(
                    best,
                    val + max_flow(valves, next_valve, &cur_opened, min_left - 2),
                );
            }
            best = cmp::max(
                best,
                max_flow(valves, next_valve, &cur_opened, min_left - 1),
            )
        }
    }
    best
}
