use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::iter::FromIterator;
use std::str;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Valve {
    name: String,
    flow: i32,
}

impl Valve {
    fn parse(s: &str) -> Self {
        let name = str::from_utf8(&s.as_bytes()[6..=7])
            .expect("string too short?")
            .to_string();
        let (_, flow) = s.split_once('=').expect("bad valve input");
        let flow = flow.parse().expect("this should be an int");
        Valve { name, flow }
    }
}

// map of valve: (flow, [connected valves])
// there's probably a way to use &str for the connected valves, but...
#[derive(Debug)]
struct ValveMap(HashMap<String, (i32, Vec<String>)>, HashMap<(String, String), i32>);

impl ValveMap {
    fn parse(s: &str) -> Self {
        // let mut valve_map = ValveMap(

        let valve_map = HashMap::from_iter(s.lines().map(|line| {
            let (valve, tunnels) = line.split_once(';').expect("bad input");
            let valve = Valve::parse(valve);
            let tunnels = match tunnels.strip_prefix(" tunnels lead to valves ") {
                None => tunnels.strip_prefix(" tunnel leads to valve ").unwrap(),
                Some(tunnels) => tunnels,
            };
            let tunnels = tunnels
                .split(", ")
                .map(str::to_string)
                .collect::<Vec<String>>();
            (valve.name.clone(), (valve.flow, tunnels))
        }));

        let mut cost_to_open: HashMap<(String, String), i32> = HashMap::new();
        for (a, (ai, tunnels)) in valve_map.iter() {
            // if *ai == 0 { continue; }
            for (b, (bi, _)) in valve_map.iter() {
                // if *bi == 0 { continue; }
                let mut q = VecDeque::from_iter(tunnels.iter().map(|name| (2, name)));
                q.push_front((1, a));
                let mut visited = HashSet::new();
                while !q.is_empty() {
                    let cur = q.pop_front().unwrap();
                    if cur.1 == b {
                        cost_to_open.insert((a.clone(), b.clone()), cur.0);
                        break;
                    } else {
                        for n in valve_map.get(cur.1).unwrap().1.iter() {
                            if visited.insert(n) {
                                q.push_back((cur.0 + 1, n));
                            }
                        }
                    }
                }
            }
        }
        dbg!(&cost_to_open);

        ValveMap(valve_map, cost_to_open)
    }
}

// TODO: symmetric map of (a, b), (b, a): distance for all non-zero valves

#[derive(Debug)]
struct ValveState {
    flow_so_far: i32,
    time_remaining: i32,
    cur: String,
    closed_valves: HashSet<String>,
}

impl Ord for ValveState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.flow_so_far.cmp(&other.flow_so_far)
    }
}

impl PartialOrd for ValveState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ValveState {
    fn eq(&self, other: &Self) -> bool {
        self.flow_so_far == other.flow_so_far
    }
}

impl Eq for ValveState {}

impl ValveState {
    fn upper_bound(&self, map: &ValveMap) -> i32 {
        // sum of:
        // (time_remaining - time_to_open) * flow
        // for each valve in closed_valves
        // TODO: does not include time_to_open -- see above for missing info
        self.flow_so_far
            + self
                .closed_valves
                .iter()
                .map(|name| map.0.get(name).unwrap().0 * 0.max(self.time_remaining - map.1.get(&(name.clone(), self.cur.clone())).unwrap()))
                // .map(|name| map.0.get(name).unwrap().0 * (self.time_remaining - 1))
                .sum::<i32>()
    }
}

fn part_1(valve_map: ValveMap) -> i32 {
    let mut best = 0;
    // initial state
    // TODO: use a BinaryHeap (may require Ord on ValveState)
    let mut to_explore = BinaryHeap::<ValveState>::new();
    to_explore.push(ValveState {
        flow_so_far: 0,
        time_remaining: 30,
        cur: "AA".to_string(),
        closed_valves: HashSet::from_iter(valve_map.0.iter().filter_map(|(name, (flow, _))| {
            if *flow > 0 {
                Some(name.clone())
            } else {
                None
            }
        })),
    });
    dbg!(&to_explore);

    while !to_explore.is_empty() {
        let cur = to_explore
            .pop()
            .expect("queue should not be empty we just checked it");
        best = best.max(cur.flow_so_far);
        // prune
        let current_upper_bound = cur.upper_bound(&valve_map);
        if best >= current_upper_bound {
            continue;
        }
        // dbg!(best, &to_explore.len());

        if cur.closed_valves.contains(&cur.cur) {
            // open valve
            // subtract time
            // add points
            // push onto to_explore
            let mut closed_valves = cur.closed_valves.clone();
            closed_valves.remove(&cur.cur);
            let time_remaining = cur.time_remaining - 1;
            let flow_so_far =
                cur.flow_so_far + time_remaining * valve_map.0.get(&cur.cur).unwrap().0;
            to_explore.push(ValveState {
                flow_so_far,
                time_remaining,
                cur: cur.cur.clone(),
                closed_valves,
            });
        }
        // for each neighbor
        // move, subtract time, push onto to_explore
        valve_map
            .0
            .get(&cur.cur)
            .unwrap()
            .1
            .iter()
            .for_each(|neighbor| {
                to_explore.push(ValveState {
                    flow_so_far: cur.flow_so_far,
                    time_remaining: cur.time_remaining - 1,
                    cur: neighbor.clone(),
                    closed_valves: cur.closed_valves.clone(),
                });
            });
    }

    dbg!(best)
}

fn main() {
    let valve_map = ValveMap::parse(include_str!("test.input.txt"));
    assert!(part_1(valve_map) == 1651);
    println!(
        "Part 1: {}",
        part_1(ValveMap::parse(include_str!("input.txt")))
    );
}
