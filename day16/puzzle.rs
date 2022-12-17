use std::collections::{HashMap, HashSet, VecDeque};
use std::str;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Valve {
    name: String,
    open: bool,
    flow: isize,
}

impl Valve {
    fn parse(s: &str) -> Self {
        let name = str::from_utf8(&s.as_bytes()[6..=7]).expect("string too short?").to_string();
        let (_, flow) = s.split_once('=').expect("bad valve input");
        let flow: isize = flow.parse().expect("this should be an int");
        Valve {
            name,
            open: false,
            flow,
        }
    }
}


// map of name: (index, [connected indexes])
#[derive(Debug)]
struct ValveMap {
    valves: Vec<Valve>,
    map: HashMap<String, (usize, Vec<usize>)>,
}

fn parse(s: &str) -> ValveMap {
    // okay...
    let valves = s.lines().map(|line| {
        let (valve, _tunnels) = line.split_once(';').expect("bad input");
        Valve::parse(valve)
    }).collect::<Vec<Valve>>();

    // okay...
    // probably a way to make this immutable with valves.iter().position()
    let mut valve_map = ValveMap{valves, map: HashMap::new()};
    for (i, valve) in valve_map.valves.iter().enumerate() {
        valve_map.map.insert(valve.name.clone(), (i, Vec::new()));
    }

    // hmmm...
    let mut tunnels = s.lines().map(|line| {
        let (_valve, tunnels) = line.split_once(';').expect("bad input");
        let tunnels = match tunnels.strip_prefix(" tunnels lead to valves ") {
            None => tunnels.strip_prefix(" tunnel leads to valve ").unwrap(),
            Some(tunnels) => tunnels,
        };
        tunnels.split(", ").map(|name| valve_map.map.get(name).unwrap().0).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    // sure...
    for (_valve, (i, t)) in valve_map.map.iter_mut() {
        t.append(&mut tunnels[*i]);
    }

    valve_map
}

fn get_next_valve(time_remaining: isize, cur: usize, valve_map: &ValveMap) -> (usize, isize) {
    let name = &valve_map.valves[cur].name;
    // keep track of visited nodes
    let mut seen = HashSet::new();
    seen.insert(&cur);
    // start with neighbors of current
    let mut q = VecDeque::new();
    for v in &valve_map.map.get(name).unwrap().1 {
        if seen.insert(v) {
            q.push_back((1, v));
        }
    }

    let mut max_priority = 0;
    let mut target = cur;
    let mut cost_to_move = 1;

    while !q.is_empty() {
        let (depth, v) = q.pop_front().unwrap();
        let valve = &valve_map.valves[*v];
        let name = &valve.name;

        // if not visited, compute priority
        let priority = (time_remaining - depth - 1) * valve.flow * (1 - valve.open as isize);
        if priority > max_priority {
            max_priority = priority;
            target = *v;
            cost_to_move = depth + 1;
        }

        // add neighbors to queue
        for v in &valve_map.map.get(name).unwrap().1 {
            if seen.insert(v) {
                q.push_back((depth + 1, v));
            }
        }
    }

    (target, cost_to_move)
}

fn part_1(mut valve_map: ValveMap) -> isize {
    let mut time_remaining = 30;
    let mut flowing = 0;
    let mut total_flow = 0;
    let mut cur = 0;

    while time_remaining > 0 {
        dbg!(&valve_map.valves[cur].name);
        let (valve_index, cost) = get_next_valve(time_remaining, cur, &valve_map);

        // move to the valve
        total_flow += flowing * cost;
        time_remaining -= cost;
        cur = valve_index;

        // open the valve
        if !valve_map.valves[cur].open {
            valve_map.valves[cur].open = true;
            flowing += valve_map.valves[cur].flow;
        }
    }

    dbg!(total_flow)
}

fn main() {
    let valve_map = parse(include_str!("test.input.txt"));
    assert!(part_1(valve_map) == 1651);
}


