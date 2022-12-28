// DP needs a overly large states to represent if a set of valve is opened (minimal bitset)
// But cutting it to 15 is managable

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::vec::Vec;

fn get_valve_map() -> (Vec<String>, Vec<i64>, HashMap<usize, Vec<usize>>) {
    let mut valves = HashMap::new();
    let mut tunnels = HashMap::new();
    let mut cnt = 1;
    let mut encoding = HashMap::new();
    encoding.insert("AA".to_string(), 0);
    let mut encoded_valve_names = vec!["AA".to_string()];

    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let parts: Vec<&str> = line_str.split("; ").collect();
        let mut valve_str = parts[0];
        valve_str = &valve_str[6..valve_str.len()];
        let valve_parts: Vec<&str> = valve_str.split(" has flow rate=").collect();
        let valve_name = valve_parts[0].to_string();
        if let None = encoding.get(&valve_name) {
            encoding.insert(valve_name.clone(), cnt);
            encoded_valve_names.push(valve_name.clone());
            cnt += 1;
        }
        let valve_val = valve_parts[1].parse::<i64>().unwrap();
        valves.insert(valve_name.clone(), valve_val);

        let tunnel_parts: Vec<&str> = parts[1].split(", ").collect();
        let mut other_valves = Vec::new();
        let tunnel_part = tunnel_parts[0];
        other_valves.push(tunnel_part[tunnel_part.len() - 2..tunnel_part.len()].to_string());
        for tunnel_part in &tunnel_parts[1..] {
            other_valves.push(tunnel_part.to_string());
        }
        for valve_name in &other_valves {
            if let None = encoding.get(valve_name) {
                encoding.insert(valve_name.clone(), cnt);
                encoded_valve_names.push(valve_name.clone());
                cnt += 1;
            }
        }
        tunnels.insert(valve_name, other_valves);
    }

    let mut encoded_valve_rate = vec![0; encoded_valve_names.len()];
    for (name, valve_val) in valves.drain() {
        let index = encoding[&name];
        encoded_valve_rate[index] = valve_val;
    }
    let mut encoded_tunnels = HashMap::new();
    for (cur_vavle, to_valves) in tunnels.drain() {
        let index_cur_valve = encoding[&cur_vavle];
        encoded_tunnels.insert(
            index_cur_valve,
            to_valves.iter().map(|vs| encoding[vs]).collect(),
        );
    }

    (encoded_valve_names, encoded_valve_rate, encoded_tunnels)
}

// Floyd–Warshall algorithm
// but cut the valve if rate is 0.
fn get_distances_valve(
    valve_map: &Vec<i64>,
    tunnel_map: &HashMap<usize, Vec<usize>>,
) -> Vec<Vec<i64>> {
    let len_valve = valve_map.len();
    let mut dist = vec![vec![i64::MAX / 2; len_valve]; len_valve];

    for (valve, other_valves) in tunnel_map {
        // unlike the Floyd, we ignored valve with flow rate 0. except "AA"
        // dfs here
        for other_valve in other_valves {
            dist[*valve][*other_valve] = 1;
        }
    }

    for k in 0..len_valve {
        for i in 0..len_valve {
            for j in 0..len_valve {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j]
                }
            }
        }
    }
    dist
}

const MAX_MINUTES: i64 = 30;
const START: &str = "AA";

#[derive(Debug, Clone)]
struct TransitionState {
    position: usize,
    time: i64,
    flow: i64,
    released_pressure: i64,
    history: Vec<usize>,
    opened: HashSet<usize>,
}

pub fn get_max_flow() {
    let (encoded_valve_names, valve_map, tunnel_map) = get_valve_map();
    let valid: HashSet<usize> = HashSet::from_iter(
        valve_map
            .iter()
            .enumerate()
            .filter(|(_, rate)| **rate > 0)
            .map(|(idx, _)| idx)
            .clone(),
    );
    let start = encoded_valve_names.iter().position(|x| x == START).unwrap();
    let dist = get_distances_valve(&valve_map, &tunnel_map);

    println!(
        "encoded_valve_names={:?} valve_map={:?}, tunnel_map={:?}",
        encoded_valve_names, valve_map, tunnel_map
    );
    // println!("dist={:?} / valid={:?}", dist, valid);
    let mut maxed_transit = TransitionState {
        position: start,
        time: 0,
        flow: 0,
        released_pressure: 0,
        history: vec![start],
        opened: HashSet::new(),
    };
    let mut q = VecDeque::new();
    q.push_back(maxed_transit.clone());
    while let Some(mut cur) = q.pop_front() {
        if valve_map[cur.position] > 0 {
            if !cur.opened.contains(&cur.position) {
                cur.opened.insert(cur.position);
                cur.released_pressure += cur.flow;
                cur.flow += valve_map[cur.position];
                cur.time += 1;
            }
        }

        if cur.released_pressure > maxed_transit.released_pressure {
            maxed_transit = cur.clone();
        }
        let mut transit = false;
        for next in &valid {
            if cur.opened.contains(next) {
                continue;
            }
            let cost = dist[cur.position][*next];
            if cur.time + cost >= MAX_MINUTES {
                continue;
            }

            transit = true;
            let mut next_state = cur.clone();
            next_state.released_pressure += cost * next_state.flow;
            next_state.time += cost;
            next_state.position = *next;
            next_state.history.push(*next);
            q.push_back(next_state);
        }
        if !transit {
            let mut next_state = cur.clone();
            next_state.released_pressure += (MAX_MINUTES - next_state.time) * next_state.flow;
            next_state.time = MAX_MINUTES;
            if next_state.released_pressure > maxed_transit.released_pressure {
                maxed_transit = next_state.clone();
            }
        }
    }
    println!("{:?}", maxed_transit);

    let mut minute = 0;
    for i in 1..maxed_transit.history.len() {
        let node = maxed_transit.history[i];
        minute += dist[maxed_transit.history[i - 1]][node];
        println!(
            "minute = {}, arrived at {}",
            minute, encoded_valve_names[node]
        );
        println!(
            "minute = {}, opening valve with pressure: {}",
            minute + 1,
            valve_map[node]
        );
        minute += 1;
    }
}
